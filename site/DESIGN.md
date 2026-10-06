# Veloce site design language

Source: opensend.cc landing page (public site; the marketing repo itself is
private, so this file records the observed design language — tokens, type,
layout rhythm, and components — as the contract for the Veloce main site).

## Tokens

| Token | Value | Use |
|---|---|---|
| `--paper` | `#f6f4ee` | page background (warm paper) |
| `--paper-2` | `#efece3` | secondary surfaces |
| `--ink` | `#17130c` | text, primary buttons |
| `--muted` | `#6f6a5e` | secondary copy, nav links |
| `--line` | `#e2ddd0` | hairline borders, section dividers |
| `--accent` | `#ff4d00` | CTAs on hover, numerals, serif-italic accents, links |
| `--card` | `#fffdf8` | cards, pills, panels |
| `--dark` / `--dark-2` | `#16130e` / `#221d15` | terminal window, code blocks, stat cards |
| `--radius` | `16px` | cards, terminal, panels, buttons `12px` |

No webfonts: system sans stack for UI, Georgia/Times serif-italic for accent
words, ui-monospace stack for code and numerals. Works fully offline.

## Type

- H1: clamp(40px, 6.4vw, 72px), weight 650, letter-spacing −0.025em, max ~16ch.
- H2: clamp(28px, 3.6vw, 40px), weight 650, letter-spacing −0.02em.
- Every headline ends with (or contains) one serif-italic accent phrase.
- Kickers: 13px, bold, uppercase, 0.14em tracking, accent color.
- Body/lede: 17–19px, muted, 1.55–1.6 line-height, max ~620px.

## Layout rhythm (mirrors opensend.cc section order)

1. Sticky blurred nav: brand left, links center, status pill + GitHub right.
2. Hero: H1 → sub → two CTAs → large product visual (terminal window here,
   screenshot there) → "Built on …" line → stack marquee (duplicated track,
   CSS animation).
3. Triptych (`Route / Render / Restore`): three cards, each with a dark code
   strip. Opensend's Send/Deliver/See, mapped to the framework's three verbs.
4. Numbered architecture (`01–04`) + side panel ("what you end up with" +
   docs link). Opensend's "Your email. Your server." block.
5. Three steps + side panel with the one cargo command.
6. Engineering proof grid (replaces the founder story — no persona invented).
7. Benchmarks: dark stat cards + a dashed honest-caveat callout (replaces
   pricing cards; no prices invented).
8. FAQ accordion (`details/summary`, `+`/`–` markers, accent on open).
9. Final CTA, then footer: brand + tagline, Product / Open source columns,
   copyright + version row.

## Components

- `.btn.solid` (ink fill → accent on hover), `.btn.ghost`.
- `.pill` status pill with green dot.
- `.term`: dark window, traffic-light dots, title bar, `pre` body with
  `.b/.g/.y/.dim` token coloring. Content must be real framework output.
- `.code`: dark strip, orange keywords, green strings, dim comments.
- `.stat`: dark card, 34px value with green unit.
- `.honest`: dashed accent-border callout for caveats. Never ship a benchmark
  section without one.
- Marquee: `.track` duplicated content, `slide` 26s linear infinite.

## Rules

- One accent per headline, always serif-italic.
- Every section: kicker → H2 → lede → content. No exceptions.
- Hairlines between sections (`1px var(--line)`), generous vertical padding
  (72–88px desktop).
- Docs links point at the Blume docs site; no second hand-rolled docs page
  may exist in this folder.
