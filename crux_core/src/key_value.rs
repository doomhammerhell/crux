//! TODO mod docs

use crate::{Command, EventConstructor};

pub struct Key(String);

/// TODO docs
pub fn read<EC, Effect, Event>(key: String, evt: EC) -> Command<Effect, Event>
where
    EC: EventConstructor<Event>,
    Effect: From<Key>,
{
    Command {
        effect: Key(key).into(),
        event_constructor: Some(Box::new(move |outcome| evt(outcome.into()))),
    }
}

pub struct KeyValue(Key, Vec<u8>);

/// TODO docs
pub fn write<EC, Effect, Event>(key: String, value: Vec<u8>, evt: EC) -> Command<Effect, Event>
where
    EC: EventConstructor<Event>,
    Effect: From<KeyValue>,
{
    Command {
        effect: KeyValue(Key(key), value).into(),
        event_constructor: Some(Box::new(move |outcome| evt(outcome.into()))),
    }
}
