use crate::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::line::{HORIZONTAL, VERTICAL};
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Frame;

// fn create_story_text(story: NewsStory) -> String {
//     let mut string = String::new();
//     string.push_str(story.title.as_str());
//     string.push_str("\n");
//     string.push_str(story.content.as_str());
//     string
// }

const TITLE_LENGTH_BREAKPOINT: usize = 40;

fn render_story_choices<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
) -> () {
    let story_list_breaker: &str = &*HORIZONTAL.repeat(TITLE_LENGTH_BREAKPOINT);
    let mut list_items: Vec<ListItem> = vec![];
    for story in &app.story_list.stories {
        let mut title: Vec<char> = story.title.chars().collect();
        let original_length = title.len();
        let mut idx = TITLE_LENGTH_BREAKPOINT as usize;
        while idx < original_length {
            if title[idx] == ' ' {
                title[idx] = '\n';
                idx += TITLE_LENGTH_BREAKPOINT + 1;
            } else {
                idx += 1;
            }
        }
        title.push('\n');
        let mut string = title.into_iter().collect::<String>();
        string.push_str(story_list_breaker);
        let text = Text::from(string);
        list_items.push(ListItem::new(text));
    }

    let widget = List::new(list_items)
        .block(Block::default().title("Stories").borders(Borders::ALL))
        .style(app.get_theme())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Green),
        )
        .highlight_symbol(VERTICAL)
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(widget, area, &mut app.story_list.state);
}

fn render_section_list<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
) -> () {
    let list_items = app
        .section_list
        .sections
        .iter()
        .map(|f| ListItem::new(f.as_str()))
        .collect::<Vec<ListItem>>();

    let widget = List::new(list_items)
        .block(Block::default().title("Sections").borders(Borders::ALL))
        .style(app.get_theme())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Green),
        )
        .highlight_symbol(VERTICAL)
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(widget, area, &mut app.section_list.state);
}

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let size = frame.size();
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![
            Constraint::Length(20),
            Constraint::Length(60),
            Constraint::Min(0),
        ])
        .split(size);

    // render story column
    render_section_list(frame, app, columns[0]);

    // render story column
    render_story_choices(frame, app, columns[1]);

    // render text area
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(&*app.story_list.selected_story().content)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(app.get_theme());
    frame.render_widget(paragraph, columns[2]);
}
