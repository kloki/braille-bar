# braille-bar

Render a percentage as a fixed-width string of braille characters.

```
 ⡀ ⡄ ⡆ ⡇ ⣇ ⣧ ⣷ ⣿
```

## Usage

```rust
use braille_bar::{braille_bar, BrailleBar};

// Quick one-liner (default: 100 points → 13 chars)
let bar = braille_bar(75.0);

// Custom width in characters
let renderer = BrailleBar::new(20);
let bar = renderer.render(75.0);

// From points (12 points → 2 chars, 100% fills exactly 12 dots)
let renderer = BrailleBar::from_points(12);
let bar = renderer.render(100.0); // "⣿⡇"

// Render raw braille units filled
let renderer = BrailleBar::new(10);
let bar = renderer.render_points(42);
```

## How it works

Each braille character represents 8 levels of fill. The renderer scales percentages proportionally to the capacity, so 100% always fills exactly the specified number of points.

When created with `new(width)`, capacity is `width * 8` — so 100% fills every character completely. When created with `from_points(n)`, capacity is exactly `n`, meaning the bar accurately represents the point count even when it doesn't align to character boundaries.
