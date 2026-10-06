---
title: Layout on a cell grid
description: Why continuous flexbox needs quantization.
---

# Layout on a cell grid

taffy computes layout in continuous `f32`, but terminals are a discrete grid
of cells. Naive truncation drifts: borders drop, siblings overlap, and flex
containers collapse to zero width.

Veloce bridges the two with a quantization pass over the taffy result:

1. The `Element` tree is mirrored into a parallel taffy tree, one node per
   element, with the terminal as definite available space.
2. Each absolute edge is rounded exactly once, so sibling edges coincide —
   rounding decisions never depend on nesting depth, and error cannot
   accumulate.
3. Every child rect is clamped inside its parent; when taffy assigned a
   positive size but rounding produced zero, the rect is forced to one cell.
4. `Text` nodes report their wrapped height through a taffy measure function,
   so wrapping participates in layout instead of overflowing it.

The practical consequence: declare flex containers, alignment, and gaps, and
the framework guarantees non-overlapping, non-collapsed cells at any size —
down to pathological 1×7 terminals.
