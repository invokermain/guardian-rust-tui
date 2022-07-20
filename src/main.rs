use std::error::Error;
use std::io;
use std::io::Stdout;

use app::app::App;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use graphics::themes::THEMES;
use tui::{backend::CrosstermBackend, Terminal};

use graphics::ui;

mod app;
mod graphics;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
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
                        app.next_story().await;
                        app.story_scroll = 0;
                    } else {
                        app.story_scroll += 1
                    }
                }
                KeyCode::Up => {
                    if app.section_idx == 0 {
                        app.prev_story();
                        app.story_scroll = 0;
                    } else if app.story_scroll >= 1 {
                        app.story_scroll -= 1
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
