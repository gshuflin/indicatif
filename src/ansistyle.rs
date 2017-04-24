use std::env;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::BTreeSet;

use term::is_a_terminal;

fn supports_styling() -> bool {
    (&env::var("CLICOLOR").unwrap_or("0".into()) != "0" &&
     is_a_terminal()) ||
    &env::var("CLICOLOR_FORCE").unwrap_or("0".into()) != "0"
}

lazy_static! {
    static ref ENABLE_STYLING: AtomicBool = AtomicBool::new(supports_styling());
}

/// Returns if ANSI styles should be used.
///
/// This returns `true` if ANSI styles should be used for formatting.  This
/// honors the `CLICOLOR` and `CLICOLOR_FORCE` environment variables and
/// defaults to the terminal default.
///
/// The `Styled` type will automatically turn itself on and off depending
/// on the value here.
pub fn should_style() -> bool {
    ENABLE_STYLING.load(Ordering::Relaxed)
}

/// Override styling.
///
/// This can be used to forcefully enable or disable coloring for this
/// library.
pub fn set_should_style(val: bool) {
    ENABLE_STYLING.store(val, Ordering::Relaxed);
}

/// An ANSI color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    #[inline(always)]
    fn ansi_num(&self) -> usize {
        match *self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
        }
    }
}

/// An ANSI style.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Style {
    Bold,
    Dim,
    Underlined,
    Blink,
    Reverse,
    Hidden,
}

impl Style {
    #[inline(always)]
    fn ansi_num(&self) -> usize {
        match *self {
            Style::Bold => 1,
            Style::Dim => 2,
            Style::Underlined => 4,
            Style::Blink => 5,
            Style::Reverse => 7,
            Style::Hidden => 8,
        }
    }
}

/// A formatting wrapper that can be styled for a terminal.
pub struct Styled<D> {
    fg: Option<Color>,
    bg: Option<Color>,
    styles: BTreeSet<Style>,
    force: Option<bool>,
    val: D,
}

/// Wraps an object for formatting for styling.
///
/// Example:
///
/// ```rust,no_run
/// format!("Hello {}", style("World").cyan());
/// ```
pub fn style<D>(val: D) -> Styled<D> {
    Styled {
        fg: None,
        bg: None,
        styles: BTreeSet::new(),
        force: None,
        val: val,
    }
}

impl<D> Styled<D> {
    /// Forces styling on or off.
    #[inline(always)]
    pub fn force_styling(mut self, value: bool) -> Styled<D> {
        self.force = Some(value);
        self
    }

    /// Sets a foreground color.
    #[inline(always)]
    pub fn fg(mut self, color: Color) -> Styled<D> {
        self.fg = Some(color);
        self
    }

    /// Sets a background color.
    #[inline(always)]
    pub fn bg(mut self, color: Color) -> Styled<D> {
        self.bg = Some(color);
        self
    }

    /// Adds a style.
    #[inline(always)]
    pub fn style(mut self, style: Style) -> Styled<D> {
        self.styles.insert(style);
        self
    }

    #[inline(always)] pub fn black(self) -> Styled<D> { self.fg(Color::Black) }
    #[inline(always)] pub fn red(self) -> Styled<D> { self.fg(Color::Red) }
    #[inline(always)] pub fn green(self) -> Styled<D> { self.fg(Color::Green) }
    #[inline(always)] pub fn yellow(self) -> Styled<D> { self.fg(Color::Yellow) }
    #[inline(always)] pub fn blue(self) -> Styled<D> { self.fg(Color::Blue) }
    #[inline(always)] pub fn magenta(self) -> Styled<D> { self.fg(Color::Magenta) }
    #[inline(always)] pub fn cyan(self) -> Styled<D> { self.fg(Color::Cyan) }
    #[inline(always)] pub fn white(self) -> Styled<D> { self.fg(Color::White) }
    #[inline(always)] pub fn on_black(self) -> Styled<D> { self.bg(Color::Black) }
    #[inline(always)] pub fn on_red(self) -> Styled<D> { self.bg(Color::Red) }
    #[inline(always)] pub fn on_green(self) -> Styled<D> { self.bg(Color::Green) }
    #[inline(always)] pub fn on_yellow(self) -> Styled<D> { self.bg(Color::Yellow) }
    #[inline(always)] pub fn on_blue(self) -> Styled<D> { self.bg(Color::Blue) }
    #[inline(always)] pub fn on_magenta(self) -> Styled<D> { self.bg(Color::Magenta) }
    #[inline(always)] pub fn on_cyan(self) -> Styled<D> { self.bg(Color::Cyan) }
    #[inline(always)] pub fn on_white(self) -> Styled<D> { self.bg(Color::White) }
    #[inline(always)] pub fn bold(self) -> Styled<D> { self.style(Style::Bold) }
    #[inline(always)] pub fn dim(self) -> Styled<D> { self.style(Style::Dim) }
    #[inline(always)] pub fn underlined(self) -> Styled<D> { self.style(Style::Underlined) }
    #[inline(always)] pub fn blink(self) -> Styled<D> { self.style(Style::Blink) }
    #[inline(always)] pub fn reverse(self) -> Styled<D> { self.style(Style::Reverse) }
    #[inline(always)] pub fn hidden(self) -> Styled<D> { self.style(Style::Hidden) }
}

macro_rules! impl_fmt {
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for Styled<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut reset = false;
                if self.force.unwrap_or_else(should_style) {
                    if let Some(fg) = self.fg {
                        write!(f, "\x1b[{}m", fg.ansi_num() + 30)?;
                        reset = true;
                    }
                    if let Some(bg) = self.bg {
                        write!(f, "\x1b[{}m", bg.ansi_num() + 40)?;
                        reset = true;
                    }
                    for style in &self.styles {
                        write!(f, "\x1b[{}m", style.ansi_num())?;
                        reset = true;
                    }
                }
                fmt::$name::fmt(&self.val, f)?;
                if reset {
                    write!(f, "\x1b[0m")?;
                }
                Ok(())
            }
        }
    }
}

impl_fmt!(Binary);
impl_fmt!(Debug);
impl_fmt!(Display);
impl_fmt!(LowerExp);
impl_fmt!(LowerHex);
impl_fmt!(Octal);
impl_fmt!(Pointer);
impl_fmt!(UpperExp);
impl_fmt!(UpperHex);
