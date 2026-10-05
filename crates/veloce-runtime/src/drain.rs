use crossbeam_channel::Receiver;

/// Priority-ordered drain: input events first, then background actions,
/// then periodic ticks. Bounded by `budget` per pass so Tier 3 stays
/// responsive to a flood of background actions (C3).
pub struct Drained<A> {
    pub inputs: Vec<crate::AppEvent>,
    pub actions: Vec<A>,
    pub ticks: usize,
}

pub fn drain_prioritized<A>(
    input_rx: &Receiver<crate::AppEvent>,
    action_rx: &Receiver<A>,
    tick_rx: &Receiver<()>,
    budget: usize,
) -> Drained<A> {
    let mut out = Drained {
        inputs: Vec::new(),
        actions: Vec::new(),
        ticks: 0,
    };
    let mut remaining = budget;
    while remaining > 0 {
        match input_rx.try_recv() {
            Ok(ev) => {
                out.inputs.push(ev);
                remaining -= 1;
            }
            Err(_) => break,
        }
    }
    while remaining > 0 {
        match action_rx.try_recv() {
            Ok(a) => {
                out.actions.push(a);
                remaining -= 1;
            }
            Err(_) => break,
        }
    }
    while remaining > 0 {
        match tick_rx.try_recv() {
            Ok(()) => {
                out.ticks += 1;
                remaining -= 1;
            }
            Err(_) => break,
        }
    }
    out
}
