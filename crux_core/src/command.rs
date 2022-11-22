/// Command captures the intent for a side-effect. Commands are return by the [`App::update`] function.
///
/// You should never create a Command yourself, instead use one of the capabilities to create a command.
/// Command is generic over `Message` in order to carry a "callback" which will be sent to the [`App::update`]
/// function when the command has been executed, and passed the resulting data.
pub struct Command<Effect, Event> {
    pub(crate) effect: Effect,
    pub(crate) event_constructor: Option<Box<dyn IntoEventConstructor<Input, Event>>>,
}

pub trait IntoEventConstructor<Event> {
    fn into(&self) -> Event;
}

// Partially applied event constructor
// which can be called without arguments
struct EventConstructorFunction<F, T, Event>
where
    F: FnOnce(T) -> Event,
{
    constructor: F,
    argument: T,
}

impl<T, Event, F> EventConstructorFunction<F, T, Event>
where
    F: FnOnce(T) -> Event,
{
    fn call(&self, output: T) -> Event {
        (self.constructor)(output)
    }
}

impl<F, T, Event> EventConstructor<Event> for EventConstructorFunction<F, T, Event>
where
    F: FnOnce(T) -> Event,
{
    fn call(&self, output: T) -> Event {
        self.call(output)
    }
}

pub trait IntoEventConstructor<F, T, Event>
where
    F: FnOnce(T) -> Event,
{
    fn into_event_constructor(self) -> EventConstructorFunction<F, T, Event>;
}

impl<F, T, Event> IntoEventConstructor<F, T, Event> for F
where
    F: FnOnce(T) -> Event,
{
    fn into_event_constructor(self, outcome: T) -> EventConstructorFunction<F, T, Event> {
        EventConstructorFunction {
            constructor: self,
            argument: outcome,
        }
    }
}
