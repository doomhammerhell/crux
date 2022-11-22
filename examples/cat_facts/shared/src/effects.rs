use crux_core::http;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum Effect {
    Http(http::Request),
    KVRead(String), // FIXME join KV together
    KVWrite(String, Vec<u8>),
    Platform,
    Render,
    Time,
}

impl From<http::Request> for Effect {
    fn from(request: http::Request) -> Self {
        Effect::Http(request)
    }
}

#[derive(Debug, Deserialize)]
pub enum Outcome {
    Cap(String),
    Http(Vec<u8>),
    KVRead(Option<Vec<u8>>), // FIXME join KV together
    KVWrite(bool),
    Platform(String),
    Time(String),
}

impl From<Outcome> for String {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Cap(it) => it,
            Outcome::Platform(it) => it,
            Outcome::Time(it) => it,
            _ => panic!("Don't know how to convert {:?} to string", outcome),
        }
    }
}

pub type Command<Event> = crux_core::Command<Effect, Outcome, Event>;

pub fn render<Event>() -> Command<Event> {
    Command {
        effect: Effect::Render,
        event_constructor: None,
    }
}
