use std::fmt::Debug;
use crate::xml::style::border::{Border, BorderElement};
use crate::xml::style::font::{Bold, Font, Italic, Underline};

pub struct Format {
    pub(crate) font: Option<Font>,
    pub(crate) border: Option<Border>,
}

pub enum FormatBorder {
    None,
    Thin,
    Medium,
    Dashed,
    Dotted,
    Thick,
    Double,
    Hair,
    MediumDashed,
    DashDot,
    MediumDashDot,
    DashDotDot,
    MediumDashDotDot,
    SlantDashDot,
}

impl FormatBorder {
    fn to_str(&self) -> &str {
        match self {
            FormatBorder::None => "none",
            FormatBorder::Thin => "thin",
            FormatBorder::Medium => "medium",
            FormatBorder::Dashed => "dashed",
            FormatBorder::Dotted => "dotted",
            FormatBorder::Thick => "thick",
            FormatBorder::Double => "double",
            FormatBorder::Hair => "hair",
            FormatBorder::MediumDashed => "mediumDashed",
            FormatBorder::DashDot => "dashDot",
            FormatBorder::MediumDashDot => "mediumDashDot",
            FormatBorder::DashDotDot => "dashDotDot",
            FormatBorder::MediumDashDotDot => "mediumDashDotDot",
            FormatBorder::SlantDashDot => "slantDashDot",
        }
    }
}

impl Format {
    pub fn new() -> Format {
        Format {
            font: None,
            border: None
        }
    }

    pub fn set_bold(mut self) -> Format {
        let font = self.font.get_or_insert(Font::default());
        font.bold = Some(Bold::default());
        self
    }
    pub fn set_italic(mut self) -> Format {
        let font = self.font.get_or_insert(Font::default());
        font.italic = Some(Italic::default());
        self
    }
    pub fn set_underline(mut self) -> Format {
        let font = self.font.get_or_insert(Font::default());
        font.underline = Some(Underline::default());
        self
    }

    pub fn set_border(mut self, format_border: FormatBorder) -> Format {
        let border = self.border.get_or_insert(Border::default());
        let style = format_border.to_str();
        border.left = BorderElement::new(style, 64);
        border.right = BorderElement::new(style, 64);
        border.top = BorderElement::new(style, 64);
        border.bottom = BorderElement::new(style, 64);
        self
    }

    pub fn set_border_left(mut self, format_border: FormatBorder) -> Format {
        let border = self.border.get_or_insert(Border::default());
        let style = format_border.to_str();
        border.left = BorderElement::new(style, 64);
        self
    }

    pub fn set_border_right(mut self, format_border: FormatBorder) -> Format {
        let border = self.border.get_or_insert(Border::default());
        let style = format_border.to_str();
        border.right = BorderElement::new(style, 64);
        self
    }

    pub fn set_border_top(mut self, format_border: FormatBorder) -> Format {
        let border = self.border.get_or_insert(Border::default());
        let style = format_border.to_str();
        border.top = BorderElement::new(style, 64);
        self
    }

    pub fn set_border_bottom(mut self, format_border: FormatBorder) -> Format {
        let border = self.border.get_or_insert(Border::default());
        let style = format_border.to_str();
        border.bottom = BorderElement::new(style, 64);
        self
    }
}