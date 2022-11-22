//! TODO mod docs

use crate::{Command, EventConstructor};

pub struct PlatformRequest;

/// TODO docs
pub fn get<EC, Effect, Event>(evt: EC) -> Command<Effect, Event>
where
    EC: EventConstructor<Event>,
    Effect: From<PlatformRequest>,
{
    Command {
        effect: PlatformRequest.into(),
        event_constructor: Some(Box::new(move |outcome| evt(outcome.into()))),
    }
}
