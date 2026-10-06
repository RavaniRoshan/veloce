# Veloce site design language

Source: the opensend.cc landing page (observed from the live site; its
marketing repo is private, so this file records the design language —
tokens, type, layout rhythm, components — as the contract for this page).

Default state is **dark**. Light mode exists behind the theme toggle and
reuses the same layout with swapped tokens.

## Tokens (dark)

| Token | Value | Use |
|---|---|---|
| `--bg` | `#000000` | outer page + rails (black, never navy) |
| `--surface` / `--surface-tint` | `#141415` / `#161616` | cards, tiles |
| `--border` / `--border-strong` | `#ffffff1a` / `#ffffff26` | hairlines, card borders |
| `--fg` / `--fg-muted` / `--fg-faint` | `#fafafa` / `#a1a1a1` / `#737373` | copy tiers |
| `--brand` / `--brand-fg` | `#ededed` / `#0a0a0a` | primary button fg/bg |
| `--success` | `#4ade80` | status dots only, never a page accent |
| `--warning` | `#fbbf24` | honest-caveat marker only |
| `--terminal-bg` / `--terminal-fg` | `#0a0a0a` / `#f5f5f5` | mock windows |
| `--border-double` / `--border-double-glow` | `#171717` / `#1717174f` | section dividers, rails |
| `--corner-mark` | `#525252` | 13px registration ticks |
| `--hatch-line` | `#ffffff0d` | outer diagonal hatch |
| radii | chip 6 · control-sm 8 · control 10 · card 14 · frame 20 · pill 999 | |
| `--column-max` | `1120px` | rails width |
| `--section-y` | `80px` desktop / `64px` mobile; hero + final CTA `112px` / `80px` | |

Headings use a white→gray text gradient; `em` uses a gray gradient
(`#8a8a8a→#5c5c5c`), italic, *not* a bright accent color. No blue/purple
glows anywhere; the final CTA glow is grayscale radial + dotgrid only.

## Type

- Sans: **Bricolage Grotesque** 200–800 (real woff2, `@font-face`, swap).
- Mono/code: **Geist Mono** (real woff2). Instrument Serif reserved, unused.
- H1 `.display`: 56px/1.05/−0.03em desktop, 40px mobile, weight 600, balanced.
- H2 `.st`: 3rem/1.08 desktop, 2rem mobile, weight 600, centered max-w-2xl.
- Body 16px/1.6, lede 18px muted, small 14px, caption 13px, code 13–14px mono.
- Never substitute Inter or a generic geometric sans.

## Layout rhythm (mirrors opensend.cc section order)

`.hatch` viewport → `.rails` 1120px → header + main + footer. Hatch visible
only in gutters wider than the rails.

1. 56px sticky header: wordmark (inline SVG + text) left, links center
   (Features, Docs, Benchmarks, FAQ), theme toggle + GitHub button right,
   hamburger under 768px.
2. Text-only centered hero (no illustration): status pill → H1 → sub →
   CTAs → proof row. Then the stack strip (label + 48s marquee, edge fade,
   pause on hover).
3. Triptych cells with 144px mock windows (terminal / schema-rows /
   dashboard skeleton) + full-width setup band + 9 capability cells with
   24px mono chips. Flush shared 1px borders, no gutters.
4. Two `.frame`+`.panel` cards: "what you end up with" rows + "how you get
   there" numbered rows, then a centered guide link.
5. Three steps with 104px clipped background numerals.
6. Open/maintainer band (96px tile, no invented persona).
7. 8-cell wall (128px min): six real dependency crates with locked versions
   + Star + Docs actions. No fabricated sponsors.
8. Two get-cards (Run it / Learn it) with checkmark rows.
9. Benchmarks: stat cells + dashed honest-caveat callout (always present).
10. FAQ: 8 rows, all closed by default, real usage answers, one-open-at-a-time.
11. Final CTA with grayscale glow. Footer with status panel (real gate
    results) + Product / Open source / Docs columns.

## Motion

- Reveal: opacity 0 → 1, translateY(16px) → 0, ~0.55s ease-out, 70ms stagger,
  once per element; disabled under `prefers-reduced-motion`.
- Buttons: `scale(.97)` press over 0.16s.
- Marquee: 48s linear infinite, duplicated track, pause on hover, 64px edge
  masks, static row under reduced motion.
- FAQ: ~0.2s expand; keyboard accessible, `aria-expanded` via `details`.

## Rules

- One gray-gradient `em` per headline; never a bright accent headline.
- Docs links point at the Blume docs site; no second hand-rolled docs page
  may exist in this folder.
- No invented copy in default state: versions from `Cargo.lock`, gate
  results from the test suite, links that resolve.
