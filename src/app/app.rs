use crate::graphics::themes::{Theme, THEMES};
use aletheia::{
    enums::{Field, OrderBy},
    GuardianContentClient,
};
use tui::widgets::ListState;

pub struct NewsStory {
    pub title: String,
    pub content: String,
    pub time: String,
    pub by: String,
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

pub struct StoryList {
    pub state: ListState,
    pub stories: Vec<NewsStory>,
}

impl StoryList {
    pub fn new() -> StoryList {
        StoryList {
            state: ListState::default(),
            stories: vec![],
        }
    }
    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn selected_story(&self) -> &NewsStory {
        let index = self.state.selected().unwrap();
        &self.stories[index]
    }
}

pub struct SectionTabs {
    pub index: usize,
    pub sections: Vec<String>,
}

impl SectionTabs {
    pub fn new() -> SectionTabs {
        SectionTabs {
            index: 0,
            sections: SECTIONS.into_iter().map(String::from).collect(),
        }
    }

    pub fn next(&mut self) {
        if self.index < SECTIONS.len() - 1 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    pub fn selected_section(&self) -> &str {
        SECTIONS[self.index]
    }
}

pub struct App {
    pub story_list: StoryList,
    pub section_tabs: SectionTabs,
    pub client: GuardianContentClient,
    pub theme_idx: usize,
    pub section_idx: u8,
    pub story_scroll: u16,
    story_page: u32,
}

async fn load_stories(
    client: &mut GuardianContentClient,
    section: &str,
    page: u32,
) -> Vec<NewsStory> {
    let response = client
        .show_fields(vec![
            Field::BodyText,
            Field::Headline,
            Field::LastModified,
            Field::Byline,
        ])
        .order_by(OrderBy::Newest)
        .section(section)
        .page_size(10)
        .page(page)
        .send()
        .await
        .unwrap();

    let mut stories: Vec<NewsStory> = vec![];

    if let Some(results) = response.results {
        results.into_iter().for_each(|f| {
            if let Some(fields) = f.fields {
                if let (
                    Some(body_text),
                    Some(headline),
                    Some(last_modified),
                    Some(by_line),
                ) = (
                    fields.body_text,
                    fields.headline,
                    fields.last_modified,
                    fields.byline,
                ) {
                    stories.push(NewsStory {
                        title: headline.trim().to_owned(),
                        content: body_text,
                        time: last_modified.format(&"%T".to_string()).to_string(),
                        by: by_line,
                    })
                }
            }
        })
    };

    stories
}

impl App {
    pub fn new() -> App {
        let api_key = env!("GUARDIAN_API_KEY", "GUARDIAN_API_KEY must be set!");
        App {
            client: GuardianContentClient::new(api_key),
            section_tabs: SectionTabs::new(),
            story_list: StoryList::new(),
            theme_idx: 0,
            section_idx: 0,
            story_scroll: 0,
            story_page: 1,
        }
    }

    pub async fn refresh(&mut self) {
        self.story_list.stories = load_stories(
            &mut self.client,
            self.section_tabs.selected_section(),
            self.story_page,
        )
        .await;

        self.story_list.state.select(Option::from(0 as usize));
        self.story_scroll = 0;
        self.story_page = 1;
    }

    async fn load_stories(&mut self) {
        self.story_list.stories.extend(
            load_stories(
                &mut self.client,
                self.section_tabs.selected_section(),
                self.story_page,
            )
            .await,
        );
    }

    pub fn theme(&self) -> &Theme {
        &THEMES[self.theme_idx]
    }

    pub fn next_section(&mut self) {
        if self.section_idx == 0 {
            self.section_idx = 1;
        }
    }

    pub fn prev_section(&mut self) {
        if self.section_idx == 1 {
            self.section_idx = 0;
        }
    }

    pub async fn next_story(&mut self) {
        let needs_more = self.story_list.state.selected().unwrap()
            >= self.story_list.stories.len() - 3;
        if needs_more {
            self.story_page += 1;
            self.load_stories().await;
        }
        self.story_list.next()
    }

    pub fn prev_story(&mut self) {
        self.story_list.previous()
    }
}
