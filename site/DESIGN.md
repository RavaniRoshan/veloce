# Veloce site design language

Source: the opensend.cc landing page (observed from the live site; its
marketing repo is private, so this file records the design language —
tokens, type, layout rhythm, components — as the contract for this page).

Default state is **dark**. Light mode exists behind the theme toggle and
reuses the same layout with swapped tokens.

## Tokens (dark)

| Token | Value | Use |
|---|---|---|
| `--bg` | `#0d0b08` dark / `#f6f1e7` light | warm black / warm paper — deliberately not pure `#000` |
| `--surface` / `--surface-tint` | `#171310` / `#1d1712` dark, `#fffdf7` / `#f1ebe0` light | cards, tiles |
| `--border` / `--border-strong` | `#ffffff17` / `#ffffff24` dark, `#1c141014` / `#1c141028` light | hairlines, card borders |
| `--fg` / `--fg-muted` / `--fg-faint` | `#faf7f1` / `#a8a094` / `#6f675c` dark, `#1c1410` / `#6b5f4f` / `#8a7d6b` light | copy tiers |
| `--brand` / `--brand-fg` | `#ededed` / `#0a0a0a` | primary button fg/bg (unchanged) |
| `--accent` / `--accent-strong` | `#ff6a2b` / `#ff8552` dark, `#d9480f` / `#b93a0b` light | **signature ember**: kickers, numerals, `+` markers, docs links, tile svg inherits currentColor |
| `--success` | `#4ade80` | status dots only, never a page accent |
| `--warning` | `#fbbf24` | honest-caveat marker only |
| `--terminal-bg` / `--terminal-fg` | `#0a0a0a` / `#f5f5f5` | mock windows |
| `--border-double` / `--border-double-glow` | `#171717` / `#1717174f` | section dividers, rails |
| `--corner-mark` | `#525252` | 13px registration ticks |
| `--hatch-line` | `#ffffff0d` | outer diagonal hatch |
| radii | chip 6 · control-sm 8 · control 10 · card 14 · frame 20 · pill 999 | |
| `--column-max` | `1120px` | rails width |
| `--section-y` | `80px` desktop / `64px` mobile; hero + final CTA `112px` / `80px` | |

Headings use a warm-white→warm-gray text gradient; `em` uses a warm gray
gradient, italic. Ember accent appears on kickers, numerals, markers, and
links only — never as a headline fill. No blue/purple glows anywhere; the
final CTA glow is grayscale radial + dotgrid only.

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

## Motion (GSAP core, CDN `gsap@3.12.5`, no plugins)

- Project defaults: `gsap.defaults({ duration: 0.55, ease: "power2.out" })`.
- Hero entrance: `gsap.from(".hero-inner > *", { autoAlpha: 0, y: 24,
  stagger: 0.09, ease: "power3.out" })` on load.
- Scroll reveals: IntersectionObserver triggers `gsap.fromTo` (autoAlpha 0→1,
  y 16→0, ~0.55s, 70ms index stagger, `overwrite: "auto"`), once per element.
- Benchmark counters: `gsap.to` object tween with `snap`-style `toFixed`
  formatting in `onUpdate`, fired once by observer at 0.4 threshold.
- FAQ: custom open/close tweening explicit `height` 0↔`auto` + autoAlpha over
  0.2s; `clearProps` on close; one open at a time.
- Theme icon swap: `fromTo` rotation −90→0 on toggle.
- Marquee stays CSS (48s linear infinite); buttons stay CSS `:active`
  `scale(.97)`. CSS is preferred where GSAP adds no control.
- Accessibility: everything gated by `gsap.matchMedia()` reduced-motion
  handling — when reduced, tweens are skipped and content renders statically.
  `.rv` elements are visible by default; GSAP applies from-states at runtime,
  so a failed CDN load degrades to a static page, never invisible content.
- Core-only rules followed: camelCase vars, transform aliases (`y`,
  `rotation`), `autoAlpha` over `opacity`, no layout-property animation.

## Logos

- Every crate/brand mark is an inline SVG `<symbol>` in the page sprite,
  referenced with `<use>` — never letter-tiles, never plain words as logos.
- `lg-ratatui` is the project's official `logo-simple.svg` (MIT,
  ratatui-org/ratatui `assets/`), wrapped in `fill="currentColor"`.
- `lg-tokio`, `lg-taffy`, `lg-crossterm`, `lg-crossbeam`, `lg-asynctrait`,
  `lg-veloce` are original stroke marks (24×24, round caps) since those
  projects publish no fetchable SVG mark. Do not hotlink lookalikes.
- Tiles size SVG via CSS (`.tile svg` 22px, `.sm-tile` 16px); marks inherit
  `currentColor` and adapt to both themes.

## Rules

- One gray-gradient `em` per headline; never a bright accent headline.
- Docs links point at the Blume docs site; no second hand-rolled docs page
  may exist in this folder.
- No invented copy in default state: versions from `Cargo.lock`, gate
  results from the test suite, links that resolve.
