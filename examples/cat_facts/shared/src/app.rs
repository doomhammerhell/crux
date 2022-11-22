use crux_core::{http, key_value, time, App, Command};
use serde::{Deserialize, Serialize};

use crate::{Effect, Outcome};
pub mod platform;

const CAT_LOADING_URL: &str = "https://c.tenor.com/qACzaJ1EBVYAAAAd/tenor.gif";
const FACT_API_URL: &str = "https://catfact.ninja/fact";
const IMAGE_API_URL: &str = "https://aws.random.cat/meow";

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct CatFact {
    fact: String,
    length: i32,
}

impl CatFact {
    fn format(&self) -> String {
        format!("{} ({} bytes)", self.fact, self.length)
    }
}

#[derive(Default)]
pub struct CatFacts {
    platform: platform::Platform,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    cat_fact: Option<CatFact>,
    cat_image: Option<CatImage>,
    platform: platform::Model,
    time: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CatImage {
    pub file: String,
}

impl Default for CatImage {
    fn default() -> Self {
        Self {
            file: CAT_LOADING_URL.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ViewModel {
    pub fact: String,
    pub image: Option<CatImage>,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    None,
    GetPlatform,
    Platform(platform::PlatformMsg),
    Clear,
    Get,
    Fetch,
    Restore,                   // restore state
    SetState(Option<Vec<u8>>), // receive the data to restore state with
    SetFact(Vec<u8>),
    SetImage(Vec<u8>),
    CurrentTime(String),
}

impl App for CatFacts {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(&self, msg: Event, model: &mut Model) -> Vec<Command<Event>> {
        match msg {
            Event::GetPlatform => Command::lift(
                self.platform
                    .update(platform::PlatformMsg::Get, &mut model.platform),
                Event::Platform,
            ),
            Event::Platform(msg) => Command::lift(
                self.platform.update(msg, &mut model.platform),
                Event::Platform,
            ),
            Event::Clear => {
                model.cat_fact = None;
                model.cat_image = None;
                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Event::None),
                    Command::render(),
                ]
            }
            Event::Get => {
                if let Some(_fact) = &model.cat_fact {
                    vec![Command::render()]
                } else {
                    model.cat_image = Some(CatImage::default());

                    vec![
                        http::get(FACT_API_URL.to_owned(), Event::SetFact), // -> Commad { body: Effect::Http(...) }
                        http::get(IMAGE_API_URL.to_string(), Event::SetImage),
                        Command::render(),
                    ]
                }
            }
            Event::Fetch => {
                model.cat_image = Some(CatImage::default());

                vec![
                    http::get(FACT_API_URL.to_owned(), Event::SetFact),
                    http::get(IMAGE_API_URL.to_string(), Event::SetImage),
                    Command::render(),
                ]
            }
            Event::SetFact(bytes) => {
                let fact = serde_json::from_slice::<CatFact>(&bytes).unwrap();
                model.cat_fact = Some(fact);

                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Event::None),
                    time::get(Event::CurrentTime),
                ]
            }
            Event::CurrentTime(iso_time) => {
                model.time = Some(iso_time);
                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Event::None),
                    Command::render(),
                ]
            }
            Event::SetImage(bytes) => {
                let image = serde_json::from_slice::<CatImage>(&bytes).unwrap();
                model.cat_image = Some(image);

                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Event::None),
                    Command::render(),
                ]
            }
            Event::Restore => {
                vec![key_value::read("state".to_string(), Event::SetState)]
            }
            Event::SetState(bytes) => {
                if let Some(bytes) = bytes {
                    if let Ok(m) = serde_json::from_slice::<Model>(&bytes) {
                        *model = m
                    };
                }

                vec![Command::render()]
            }
            Event::None => vec![],
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        let fact = match (&model.cat_fact, &model.time) {
            (Some(fact), Some(time)) => format!("Fact from {}: {}", time, fact.format()),
            (Some(fact), _) => fact.format(),
            _ => "No fact".to_string(),
        };

        let platform = self.platform.view(&model.platform).platform;

        ViewModel {
            platform: format!("Hello {}", platform),
            fact,
            image: model.cat_image.clone(),
        }
    }
}
