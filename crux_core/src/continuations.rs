use crate::{EventConstructor, Request, Response};
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

type Store<Event> = HashMap<[u8; 16], Box<dyn EventConstructor<Event>>>;
pub(crate) struct ContinuationStore<Event>(RwLock<Store<Event>>);

impl<Event> Default for ContinuationStore<Event> {
    fn default() -> Self {
        Self(RwLock::new(HashMap::new()))
    }
}

impl<Event> ContinuationStore<Event> {
    pub(crate) fn pause<Effect>(
        &self,
        effect: Effect,
        event_ctor: Option<Box<dyn EventConstructor<Event>>>,
    ) -> Request<Effect> {
        let uuid = *Uuid::new_v4().as_bytes();

        if let Some(evt_constructor) = event_ctor {
            self.0
                .write()
                .expect("Continuation RwLock poisoned.")
                .insert(uuid, evt_constructor);
        };

        Request {
            uuid: uuid.to_vec(),
            effect,
        }
    }

    pub(crate) fn resume<Outcome>(&self, response: Response<Outcome>) -> Event {
        let Response {
            uuid,
            outcome: body,
        } = response;
        let cont = self
            .0
            .write()
            .expect("Continuation RwLock poisoned.")
            .remove(&uuid[..])
            .unwrap_or_else(|| panic!("Continuation with UUID {:?} not found.", uuid));

        cont(body.into())
    }
}
