//! TODO mod docs

use serde::{Deserialize, Serialize};

use crate::{Command, IntoEventConstructor};

#[derive(Serialize)]
pub enum Method {
    Get,
    Post,
    Update,
    Delete,
}

#[derive(Serialize)]
pub struct Request {
    pub url: String,
    pub method: Method,
}

#[derive(Deserialize)]
pub struct Response {
    pub body: Vec<u8>,
}

/// TODO docs
pub fn get<F, Effect, Event>(url: String, evt: F) -> Command<Effect, Event>
where
    F: FnOnce(Vec<u8>) -> Event + Send + Sync + 'static,
    Effect: From<Request>,
{
    let effect = Request {
        url,
        method: Method::Get,
    }
    .into();

    Command {
        effect,
        event_constructor: Some(Box::new(|output| evt.into_event_constructor(output))),
    }
}
