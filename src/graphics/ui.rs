use crate::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Modifier;
use tui::symbols::line::{HORIZONTAL, VERTICAL};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap};
use tui::Frame;

fn wrap_string(string: String, split_len: usize) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    let original_length = chars.len();

    let mut idx = split_len;
    let max_lookback = split_len / 5;

    while idx < original_length {
        let mut line_wrapped = false;

        for lookback in 0..max_lookback {
            if chars[idx - lookback] == ' ' {
                chars[idx - lookback] = '\n';
                line_wrapped = true;
                break;
            }
        }

        if !line_wrapped {
            chars.insert(idx - 1, '\n');
            chars.insert(idx - 1, '-');
        }

        idx += split_len;
    }
    chars.into_iter().collect::<String>()
}

fn render_story_choices<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
) -> () {
    let story_list_breaker: &str = &*HORIZONTAL.repeat((area.width - 4) as usize);
    let mut list_items: Vec<ListItem> = Vec::new();
    for story in &app.story_list.stories {
        let mut wrapped = wrap_string(story.title.clone(), (area.width - 4) as usize);
        wrapped.push('\n');
        wrapped.push_str(story_list_breaker);
        list_items.push(ListItem::new(wrapped));
    }

    let style = match app.section_idx {
        0 => app.theme().active(),
        _ => app.theme().primary(),
    };

    let widget = List::new(list_items)
        .block(
            Block::default()
                .title("Stories")
                .borders(Borders::ALL)
                .style(style),
        )
        .style(app.theme().primary())
        .highlight_style(app.theme().active().add_modifier(Modifier::ITALIC))
        .highlight_symbol(VERTICAL)
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(widget, area, &mut app.story_list.state);
}

fn render_sections<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) -> () {
    let tabs = app
        .section_tabs
        .sections
        .clone()
        .into_iter()
        .clone()
        .map(Spans::from)
        .collect();

    let widget = Tabs::new(tabs)
        .block(Block::default().title("Sections").borders(Borders::ALL))
        .style(app.theme().primary())
        .highlight_style(app.theme().active().add_modifier(Modifier::UNDERLINED))
        .select(app.section_tabs.index);

    frame.render_widget(widget, area);
}

fn render_story<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) -> () {
    let style = match app.section_idx {
        1 => app.theme().active(),
        _ => app.theme().primary(),
    };

    let block = Block::default().borders(Borders::ALL).style(style);
    let paragraph = Paragraph::new(&*app.story_list.selected_story().content)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(app.theme().primary())
        .scroll((app.story_scroll, 0));
    frame.render_widget(paragraph, area);
}

fn render_controls<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) -> () {
    let text = Spans(vec![
        Span::styled("<left|right>", app.theme().secondary()),
        Span::from(" change section "),
        Span::styled("<up|down>", app.theme().secondary()),
        Span::from(" scroll section "),
        Span::styled("<f5>", app.theme().secondary()),
        Span::from(" refresh "),
        Span::styled("<f9>", app.theme().secondary()),
        Span::from(format!(" change theme (current: '{}') ", app.theme().name)),
        Span::styled("<esc>", app.theme().secondary()),
        Span::from(" quit "),
    ]);
    let paragraph = Paragraph::new(text)
        .style(app.theme().primary())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let size = frame.size();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(rows[1]);

    render_sections(frame, app, rows[0]);
    render_story_choices(frame, app, columns[0]);
    render_story(frame, app, columns[1]);
    render_controls(frame, app, rows[2]);
}
