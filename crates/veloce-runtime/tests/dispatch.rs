//! C1 channel semantics, C3 priority, C4 key-latency probe, C5 no-deadlock.
use std::time::{Duration, Instant};

use veloce_runtime::{drain_prioritized, AppEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
enum A {
    Bg(u32),
}

#[test]
fn priority_input_over_actions_over_ticks() {
    let (in_tx, in_rx) = crossbeam_channel::bounded::<AppEvent>(64);
    let (act_tx, act_rx) = crossbeam_channel::bounded::<A>(64);
    let (tick_tx, tick_rx) = crossbeam_channel::bounded::<()>(64);

    for i in 0..4 {
        act_tx.send(A::Bg(i)).unwrap();
        tick_tx.send(()).unwrap();
    }
    in_tx.send(AppEvent::Resize(80, 24)).unwrap();
    in_tx.send(AppEvent::Resize(100, 30)).unwrap();

    let drained = drain_prioritized(&in_rx, &act_rx, &tick_rx, 64);
    assert_eq!(drained.inputs.len(), 2);
    assert_eq!(drained.actions.len(), 4);
    assert_eq!(drained.ticks, 4);

    // Budget-bound pass: inputs fully drained even when budget is tight,
    // because inputs take first claim on the budget.
    for i in 0..4 {
        act_tx.send(A::Bg(i)).unwrap();
        tick_tx.send(()).unwrap();
    }
    in_tx.send(AppEvent::Resize(1, 1)).unwrap();
    in_tx.send(AppEvent::Resize(2, 2)).unwrap();
    let drained = drain_prioritized(&in_rx, &act_rx, &tick_rx, 3);
    assert_eq!(drained.inputs.len(), 2);
    assert_eq!(drained.actions.len(), 1);
    assert_eq!(drained.ticks, 0);
}

#[test]
fn ten_k_interleaved_events_no_deadlock() {
    let (in_tx, in_rx) = crossbeam_channel::bounded::<AppEvent>(1024);
    let (act_tx, act_rx) = crossbeam_channel::bounded::<A>(1024);
    let (tick_tx, tick_rx) = crossbeam_channel::bounded::<()>(1024);

    let in_prod = {
        let in_tx = in_tx.clone();
        std::thread::spawn(move || {
            for i in 0..10_000u32 {
                in_tx.send(AppEvent::Resize(i as u16, i as u16)).unwrap();
            }
        })
    };
    let act_prod = {
        let act_tx = act_tx.clone();
        std::thread::spawn(move || {
            for i in 0..10_000u32 {
                act_tx.send(A::Bg(i)).unwrap();
            }
        })
    };
    let tick_prod = {
        let tick_tx = tick_tx.clone();
        std::thread::spawn(move || {
            for _ in 0..10_000 {
                tick_tx.send(()).unwrap();
            }
        })
    };

    let consumer = std::thread::spawn(move || {
        let mut inputs = 0;
        let mut actions = 0;
        let mut ticks = 0;
        loop {
            let d = drain_prioritized(&in_rx, &act_rx, &tick_rx, 256);
            inputs += d.inputs.len();
            actions += d.actions.len();
            ticks += d.ticks;
            if inputs == 10_000 && actions == 10_000 && ticks == 10_000 {
                break;
            }
        }
        (inputs, actions, ticks)
    });

    in_prod.join().unwrap();
    act_prod.join().unwrap();
    tick_prod.join().unwrap();
    drop(in_tx);
    drop(act_tx);
    drop(tick_tx);
    let (i, a, t) = consumer.join().unwrap();
    assert_eq!((i, a, t), (10_000, 10_000, 10_000));
}

#[test]
fn key_echo_not_blocked_by_1000_actions_per_sec() {
    // C4 measured probe: while a producer enqueues actions at ~1000/s,
    // inject "key" events and measure the enqueue-to-drain latency p95.
    let (in_tx, in_rx) = crossbeam_channel::bounded::<AppEvent>(1024);
    let (act_tx, act_rx) = crossbeam_channel::bounded::<A>(1024);
    let (_tick_tx, tick_rx) = crossbeam_channel::bounded::<()>(16);

    let producer = std::thread::spawn(move || {
        for i in 0..1000u32 {
            let _ = act_tx.try_send(A::Bg(i));
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    let mut latencies = Vec::new();
    let keys = 100;
    for i in 0..keys {
        let sent = Instant::now();
        in_tx.send(AppEvent::Resize(i as u16, i as u16)).unwrap();
        // drain loop mimicking Tier 3
        loop {
            let d = drain_prioritized(&in_rx, &act_rx, &tick_rx, 64);
            if !d.inputs.is_empty() {
                latencies.push(sent.elapsed());
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    producer.join().unwrap();
    // drain remaining actions so nothing is left
    loop {
        let d = drain_prioritized(&in_rx, &act_rx, &tick_rx, 256);
        if d.inputs.is_empty() && d.actions.is_empty() {
            break;
        }
    }
    latencies.sort();
    let p95 = latencies[(keys * 95) / 100];
    let max = latencies[latencies.len() - 1];
    println!("KEY_LATENCY_P95_US={}", p95.as_micros());
    println!("KEY_LATENCY_MAX_US={}", max.as_micros());
    assert!(
        p95.as_millis() < 50,
        "key latency p95 {}ms >= 50ms budget",
        p95.as_millis()
    );
}
