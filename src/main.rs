use std::error::Error;
use std::io;
use std::io::Stdout;

use aletheia::enums::{Field, OrderBy};
use aletheia::GuardianContentClient;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::style::{Color, Style};
use tui::widgets::ListState;
use tui::{backend::CrosstermBackend, Terminal};

use graphics::ui;

use crate::news::news::NewsStory;

mod graphics;
mod news;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

const SECTIONS: [&str; 8] = [
    "world",
    "sport",
    "technology",
    "science",
    "culture",
    "lifeandstyle",
    "money",
    "weather",
];

struct StoryList {
    pub state: ListState,
    pub stories: Vec<NewsStory>,
}

struct Theme {
    name: &'static str,
    bg: Color,
    primary: Color,
    secondary: Color,
    active: Color,
}

impl Theme {
    fn primary(&self) -> Style {
        Style::default().fg(self.primary).bg(self.bg)
    }
    fn secondary(&self) -> Style {
        Style::default().fg(self.secondary).bg(self.bg)
    }
    fn active(&self) -> Style {
        Style::default().fg(self.active).bg(self.bg)
    }
}

const THEMES: [Theme; 3] = [
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

impl StoryList {
    fn new() -> StoryList {
        StoryList {
            state: ListState::default(),
            stories: vec![],
        }
    }
    fn next(&mut self) -> () {
        let i = match self.state.selected() {
            Some(i) => {
                if i == self.stories.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) -> () {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn selected_story(&self) -> &NewsStory {
        let index = self.state.selected().unwrap();
        &self.stories[index]
    }
}

struct SectionTabs {
    pub index: usize,
    pub sections: Vec<String>,
}

impl SectionTabs {
    fn new() -> SectionTabs {
        SectionTabs {
            index: 0,
            sections: SECTIONS.into_iter().map(String::from).collect(),
        }
    }

    fn next(&mut self) -> () {
        if self.index < SECTIONS.len() - 1 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    fn selected_section(&self) -> &str {
        SECTIONS[self.index]
    }
}

pub struct App {
    story_list: StoryList,
    section_tabs: SectionTabs,
    client: GuardianContentClient,
    theme_idx: usize,
    section_idx: u8,
    story_scroll: u16,
}

impl App {
    fn new() -> App {
        let api_key = env!("GUARDIAN_API_KEY", "GUARDIAN_API_KEY must be set!");
        App {
            client: GuardianContentClient::new(api_key),
            section_tabs: SectionTabs::new(),
            story_list: StoryList::new(),
            theme_idx: 0,
            section_idx: 0,
            story_scroll: 0,
        }
    }

    async fn refresh(&mut self) -> () {
        let response = self
            .client
            .show_fields(vec![Field::BodyText, Field::Headline])
            .order_by(OrderBy::Newest)
            .section(self.section_tabs.selected_section())
            .page_size(10)
            .send()
            .await
            .unwrap();

        let mut stories: Vec<NewsStory> = vec![];

        if let Some(results) = response.results {
            results.into_iter().for_each(|f| {
                if let Some(fields) = f.fields {
                    if let (Some(body_text), Some(headline)) =
                        (fields.body_text, fields.headline)
                    {
                        stories.push(NewsStory {
                            title: headline,
                            content: body_text,
                        })
                    }
                }
            })
        };

        self.story_list.stories = stories;

        self.story_list.state.select(Option::from(0 as usize));
        self.story_scroll = 0;
    }

    fn theme(&self) -> &Theme {
        &THEMES[self.theme_idx]
    }

    fn next_section(&mut self) -> () {
        if self.section_idx == 0 {
            self.section_idx = 1;
        }
    }

    fn prev_section(&mut self) -> () {
        if self.section_idx == 1 {
            self.section_idx = 0;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal =
        setup_terminal().expect("There was a problem setting up the terminal!");

    let mut app = App::new();
    app.refresh().await;

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Down => {
                    if app.section_idx == 0 {
                        app.story_list.next();
                        app.story_scroll = 0;
                    } else {
                        app.story_scroll += 1
                    }
                }
                KeyCode::Up => {
                    if app.section_idx == 0 {
                        app.story_list.previous();
                        app.story_scroll = 0;
                    } else {
                        if app.story_scroll >= 1 {
                            app.story_scroll -= 1
                        }
                    }
                }
                KeyCode::Tab => {
                    app.section_tabs.next();
                    app.refresh().await;
                }
                KeyCode::Left => {
                    app.prev_section();
                }
                KeyCode::Right => {
                    app.next_section();
                }
                KeyCode::F(5) => {
                    app.refresh().await;
                }
                KeyCode::F(9) => {
                    if app.theme_idx < THEMES.len() - 1 {
                        app.theme_idx += 1;
                    } else {
                        app.theme_idx = 0;
                    }
                }
                _ => {}
            }
        }
    }
}
