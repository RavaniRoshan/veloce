---
title: Decisions
description: Every judgment call, recorded.
---

# Decisions

- Contract resolved per kickoff: single `update(&mut self, action, cx: &mut Context<A>)`;
  `Command` return removed; `Context::dispatcher()` returns cloneable `Dispatcher<A>` for
  background tasks; `Context::dispatch()` is the same sink; `VeloceApp` owns the Tokio
  Runtime and Context holds a private Handle (never exposed in public API).
- `docs/PRD.md` was missing; kickoff brief treated as PRD text.
- Ingestion backpressure: resize/focus coalesce via try_send; keys/mouse/paste block send.
- Ticker must be created inside a Tokio runtime context.
- Loop test fix: `Ok(false)` from poll = timeout, continue (not exit).

- During hydration the view is owned by its load task; keys/actions are held until Ready (fallback screen owns input). Documented in ViewAdapter.
- Async trait load uses #[async_trait::async_trait] so futures are Send and the impl syntax matches the reference example.
- Action channel carries BoxedAction (Box<dyn Any+Send>); typed Dispatcher<A> boxes actions; ViewAdapter downcasts. No Action type escapes into Router/std storage.
- Element::Overlay modal rect = centered 60%x60% via taffy Absolute positioning; backdrop preserves underlying symbols, darkens fg.
- 'q' is a global quit key in VeloceApp::run (documented); views receive other keys first only when returning true is false — VeloceApp checks 'q' before routing.
