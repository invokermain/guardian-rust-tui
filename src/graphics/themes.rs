use tui::style::{Color, Style};

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub primary: Color,
    pub secondary: Color,
    pub active: Color,
}

impl Theme {
    pub fn primary(&self) -> Style {
        Style::default().fg(self.primary).bg(self.bg)
    }
    pub fn secondary(&self) -> Style {
        Style::default().fg(self.secondary).bg(self.bg)
    }
    pub fn active(&self) -> Style {
        Style::default().fg(self.active).bg(self.bg)
    }
}

pub const THEMES: [Theme; 3] = [
    Theme {
        name: "default",
        bg: Color::Rgb(0, 0, 50),
        primary: Color::White,
        secondary: Color::Yellow,
        active: Color::Green,
    },
    Theme {
        name: "paper",
        bg: Color::Rgb(255, 255, 230),
        primary: Color::Rgb(75, 75, 50),
        secondary: Color::Rgb(145, 145, 75),
        active: Color::Black,
    },
    Theme {
        name: "party",
        bg: Color::Rgb(255, 0, 255),
        primary: Color::Rgb(255, 255, 0),
        secondary: Color::White,
        active: Color::Gray,
    },
];
