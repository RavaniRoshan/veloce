use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use veloce::prelude::*;

fn render_frame(n: usize) -> String {
    let el = Flex::column()
        .child(
            Flex::row()
                .border(BorderStyle::Rounded)
                .padding(1)
                .child(Text::new("Service: auth-service-v2"))
                .child(Spacer::grow())
                .child(Text::new("Status: Live")),
        )
        .child(
            ScrollView::new()
                .flex_grow(1.0)
                .children((0..n).map(|i| Text::new(format!("log line {i}")).into_element())),
        )
        .into_element();
    veloce_render::render_to_string(&el, 120, 40)
}

fn bench_frame_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cost");
    for n in [10, 100, 500, 2000] {
        group.bench_with_input(BenchmarkId::new("children", n), &n, |b, &n| {
            b.iter(|| render_frame(n))
        });
    }
    group.finish();
}

fn bench_cold_boot(c: &mut Criterion) {
    c.bench_function("cold_boot_first_frame", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let interval = rt.block_on(async {
                let mut i = veloce_runtime::tick_interval();
                i.tick().await;
                i
            });
            drop(interval);
            let t = std::time::Instant::now();
            let el = Flex::column().child(Text::new("hi")).into_element();
            let _ = veloce_render::render_to_string(&el, 80, 24);
            t.elapsed()
        })
    });
}

criterion_group!(benches, bench_frame_cost, bench_cold_boot);
criterion_main!(benches);
