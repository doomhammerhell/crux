use crux_core::{App, Command};
use serde::{Deserialize, Serialize};

use crate::effects::{Effect, Outcome};

#[derive(Default)]
pub struct Platform;

#[derive(Default, Serialize, Deserialize)]
pub struct Model {
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub enum PlatformMsg {
    Get,
    Set(String),
}

impl App for Platform {
    type Event = PlatformMsg;
    type Model = Model;
    type ViewModel = Model;
    type AppRequest = ();

    fn update(
        &self,
        msg: PlatformMsg,
        model: &mut Model,
    ) -> Vec<Command<Effect, Outcome, PlatformMsg>> {
        match msg {
            PlatformMsg::Get => vec![platform::get(Box::new(PlatformMsg::Set))],
            PlatformMsg::Set(platform) => {
                model.platform = platform;
                vec![Command::render()]
            }
        }
    }

    fn view(&self, model: &Model) -> Model {
        Model {
            platform: model.platform.clone(),
        }
    }
}
