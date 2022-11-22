//! TODO mod docs

use crate::{Command, EventConstructor};

pub struct TimeRequest;

/// TODO docs
pub fn get<EC, Effect, Outcome, Event>(evt: EC) -> Command<Effect, Event>
where
    EC: EventConstructor<Event>,
    Effect: From<TimeRequest>,
{
    Command {
        effect: TimeRequest.into(),
        event_constructor: Some(Box::new(move |outcome: Outcome| evt(outcome.into()))),
    }
}
