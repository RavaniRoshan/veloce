# Decisions

- Contract resolved per kickoff: single `update(&mut self, action, cx: &mut Context<A>)`;
  `Command` return removed; `Context::dispatcher()` returns cloneable `Dispatcher<A>` for
  background tasks; `Context::dispatch()` is the same sink; `VeloceApp` owns the Tokio
  Runtime and Context holds a private Handle (never exposed in public API).
- `docs/PRD.md` was missing; kickoff brief treated as PRD text.
- Ingestion backpressure: resize/focus coalesce via try_send; keys/mouse/paste block send.
- Ticker must be created inside a Tokio runtime context.
- Loop test fix: `Ok(false)` from poll = timeout, continue (not exit).
