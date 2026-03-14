pub const BRAILLE: [char; 9] = [' ', '⡀', '⡄', '⡆', '⡇', '⣇', '⣧', '⣷', '⣿'];

pub const DEFAULT_POINTS: usize = 100;

/// Render a percentage as a braille bar with the default width.
///
/// ```
/// use braille_bar::braille_bar;
///
/// let bar = braille_bar(75.0);
/// assert_eq!(bar.chars().count(), 13);
/// ```
pub fn braille_bar(pct: f64) -> String {
    BrailleBar::default().render(pct)
}

/// A configurable braille bar renderer.
///
/// ```
/// use braille_bar::BrailleBar;
///
/// // Custom width in characters
/// let renderer = BrailleBar::new(20);
/// let bar = renderer.render(75.0);
/// assert_eq!(bar.chars().count(), 20);
///
/// // From points (100 points → ceil(100/8) = 13 chars)
/// let renderer = BrailleBar::from_points(100);
/// let bar = renderer.render(50.0);
/// assert_eq!(bar.chars().count(), 13);
///
/// // Render raw braille units filled
/// let renderer = BrailleBar::new(10);
/// let bar = renderer.render_points(42);
/// assert_eq!(bar.chars().count(), 10);
/// ```
pub struct BrailleBar {
    capacity: usize,
}

impl BrailleBar {
    /// Create a renderer with the given width in characters.
    ///
    /// A width of 0 is clamped to 1.
    ///
    /// ```
    /// use braille_bar::BrailleBar;
    ///
    /// let renderer = BrailleBar::new(5);
    /// assert_eq!(renderer.render(100.0), "⣿⣿⣿⣿⣿");
    /// ```
    pub fn new(width: usize) -> Self {
        Self {
            capacity: width.max(1) * 8,
        }
    }

    /// Create a renderer sized to fit `points` braille units.
    ///
    /// The width in characters is `ceil(points / 8)`, minimum 1.
    ///
    /// ```
    /// use braille_bar::BrailleBar;
    ///
    /// // 12 points → ceil(12/8) = 2 chars wide, but 100% only fills 12 dots
    /// let renderer = BrailleBar::from_points(12);
    /// assert_eq!(renderer.render(0.0).chars().count(), 2);
    /// assert_eq!(renderer.render(100.0), "⣿⡇");
    /// ```
    pub fn from_points(points: usize) -> Self {
        Self {
            capacity: points.max(1),
        }
    }

    /// Render a percentage (clamped to 0.0..=100.0) as a braille bar.
    ///
    /// ```
    /// use braille_bar::BrailleBar;
    ///
    /// let renderer = BrailleBar::new(4);
    /// assert_eq!(renderer.render(50.0), "⣿⣿  ");
    /// assert_eq!(renderer.render(100.0), "⣿⣿⣿⣿");
    /// ```
    pub fn render(&self, pct: f64) -> String {
        let filled = (pct.clamp(0.0, 100.0) / 100.0 * self.capacity as f64).round() as usize;
        self.render_filled(filled)
    }

    /// Render a raw number of braille units filled.
    ///
    /// Points exceeding capacity render as a full bar.
    ///
    /// ```
    /// use braille_bar::BrailleBar;
    ///
    /// let renderer = BrailleBar::new(2);
    /// assert_eq!(renderer.render_points(10), "⣿⡄");
    /// // overflow clamps to full
    /// assert_eq!(renderer.render_points(999), "⣿⣿");
    /// ```
    pub fn render_points(&self, points: usize) -> String {
        let filled = points.min(self.capacity);
        self.render_filled(filled)
    }

    fn width(&self) -> usize {
        self.capacity.div_ceil(8)
    }

    fn render_filled(&self, filled: usize) -> String {
        let width = self.width();
        let full_chars = filled / 8;
        let remainder = filled % 8;

        let mut bar = String::with_capacity(width * 4);
        for _ in 0..full_chars {
            bar.push(BRAILLE[8]);
        }
        if full_chars < width {
            bar.push(BRAILLE[remainder]);
            for _ in (full_chars + 1)..width {
                bar.push(BRAILLE[0]);
            }
        }
        bar
    }
}

impl Default for BrailleBar {
    fn default() -> Self {
        Self::from_points(DEFAULT_POINTS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_percent_is_all_spaces() {
        let bar = BrailleBar::new(5).render(0.0);
        assert_eq!(bar, "     ");
        assert_eq!(bar.chars().count(), 5);
    }

    #[test]
    fn hundred_percent_is_all_full() {
        let bar = BrailleBar::new(5).render(100.0);
        assert_eq!(bar, "⣿⣿⣿⣿⣿");
        assert_eq!(bar.chars().count(), 5);
    }

    #[test]
    fn fifty_percent() {
        let bar = BrailleBar::new(4).render(50.0);
        // 4 chars * 8 = 32 units, 50% = 16 units = 2 full chars
        assert_eq!(bar, "⣿⣿  ");
    }

    #[test]
    fn negative_clamps_to_zero() {
        let bar = BrailleBar::new(3).render(-10.0);
        assert_eq!(bar, "   ");
    }

    #[test]
    fn over_hundred_clamps_to_full() {
        let bar = BrailleBar::new(3).render(150.0);
        assert_eq!(bar, "⣿⣿⣿");
    }

    #[test]
    fn default_is_100_points() {
        let bar = BrailleBar::default();
        // 100 points → ceil(100/8) = 13 chars
        assert_eq!(bar.render(0.0).chars().count(), 13);
    }

    #[test]
    fn braille_bar_function_uses_default() {
        let bar = braille_bar(0.0);
        assert_eq!(bar.chars().count(), 13);
    }

    #[test]
    fn from_points_100() {
        // ceil(100/8) = 13 chars, capacity = 100
        let renderer = BrailleBar::from_points(100);
        assert_eq!(renderer.render(0.0).chars().count(), 13);
        // 100% fills exactly 100 units = 12 full + remainder 4
        assert_eq!(renderer.render(100.0), "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇");
    }

    #[test]
    fn from_points_12() {
        // ceil(12/8) = 2 chars, capacity = 12
        let renderer = BrailleBar::from_points(12);
        assert_eq!(renderer.render(0.0).chars().count(), 2);
        // 100% fills 12 units = 1 full + remainder 4
        assert_eq!(renderer.render(100.0), "⣿⡇");
    }

    #[test]
    fn from_points_8() {
        // ceil(8/8) = 1 char, capacity = 8
        let renderer = BrailleBar::from_points(8);
        assert_eq!(renderer.render(0.0).chars().count(), 1);
        assert_eq!(renderer.render(100.0), "⣿");
    }

    #[test]
    fn from_points_zero() {
        // clamped to 1 point, ceil(1/8) = 1 char
        let renderer = BrailleBar::from_points(0);
        assert_eq!(renderer.render(0.0).chars().count(), 1);
    }

    #[test]
    fn render_points_basic() {
        let renderer = BrailleBar::new(2);
        // 2 chars = 16 capacity, fill 10 = 1 full + remainder 2
        let bar = renderer.render_points(10);
        assert_eq!(bar, "⣿⡄");
    }

    #[test]
    fn render_points_overflow_clamps() {
        let renderer = BrailleBar::new(2);
        let bar = renderer.render_points(999);
        assert_eq!(bar, "⣿⣿");
    }

    #[test]
    fn render_points_zero() {
        let renderer = BrailleBar::new(3);
        assert_eq!(renderer.render_points(0), "   ");
    }

    #[test]
    fn width_one_works() {
        let renderer = BrailleBar::new(1);
        assert_eq!(renderer.render(0.0), " ");
        assert_eq!(renderer.render(100.0), "⣿");
        // 50% of 8 = 4 units
        assert_eq!(renderer.render(50.0), "⡇");
    }

    #[test]
    fn width_zero_clamps_to_one() {
        let renderer = BrailleBar::new(0);
        assert_eq!(renderer.render(0.0).chars().count(), 1);
        assert_eq!(renderer.render(100.0), "⣿");
    }

    #[test]
    fn output_length_is_always_width() {
        let renderer = BrailleBar::new(10);
        for pct in [0.0, 1.0, 25.0, 33.3, 50.0, 75.0, 99.0, 100.0] {
            assert_eq!(renderer.render(pct).chars().count(), 10, "pct={pct}");
        }
    }

    #[test]
    fn scaling_100_percent_fills_any_width() {
        for w in 1..=50 {
            let bar = BrailleBar::new(w).render(100.0);
            assert!(bar.chars().all(|c| c == '⣿'), "width={w} not fully filled");
        }
    }
}
