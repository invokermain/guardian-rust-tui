use std::error::Error;
use std::io::Stdout;
use std::{io, thread, time::Duration};

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

const TICK_DURATION: Duration = Duration::from_millis(100);

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

const SECTIONS: [&str; 2] = ["world", "sport"];

struct StoryList {
    pub state: ListState,
    pub stories: Vec<NewsStory>,
}

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

struct SectionList {
    pub state: ListState,
    pub sections: Vec<String>,
}

impl SectionList {
    fn new() -> SectionList {
        let mut out = SectionList {
            state: ListState::default(),
            sections: SECTIONS
                .into_iter()
                .map(|f| String::from(f))
                .collect::<Vec<String>>(),
        };
        out.state.select(Option::from(0 as usize));
        out
    }

    fn next(&mut self) -> () {
        let i = match self.state.selected() {
            Some(i) => {
                if i == SECTIONS.len() - 1 {
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

    fn selected_section(&self) -> &str {
        let index = self.state.selected().unwrap();
        SECTIONS[index]
    }
}

pub struct App {
    story_list: StoryList,
    section_list: SectionList,
    client: GuardianContentClient,
    theme: String,
}

impl App {
    fn new() -> App {
        let api_key = env!("GUARDIAN_API_KEY", "GUARDIAN_API_KEY must be set!");
        App {
            client: GuardianContentClient::new(api_key),
            section_list: SectionList::new(),
            story_list: StoryList::new(),
            theme: String::from("default"),
        }
    }

    async fn refresh(&mut self) -> () {
        let response = self
            .client
            .show_fields(vec![Field::BodyText, Field::Headline])
            .order_by(OrderBy::Newest)
            .section(self.section_list.selected_section())
            .page_size(5)
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
    }

    fn get_theme(&self) -> Style {
        match self.theme.as_str() {
            "default" => Style::default().fg(Color::White).bg(Color::Rgb(0, 0, 50)),
            _ => Style::default(),
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
        if event::poll(TICK_DURATION)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.story_list.next(),
                    KeyCode::Up => app.story_list.previous(),
                    KeyCode::Left => {
                        app.section_list.next();
                        app.refresh().await;
                    }
                    KeyCode::Right => {
                        app.section_list.previous();
                        app.refresh().await;
                    }
                    _ => {}
                }
            }
        }
        thread::sleep(TICK_DURATION);
    }
}
