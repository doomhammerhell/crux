//! Testing support for unit testing Crux apps.
use std::{fmt, rc::Rc};

use crate::{
    capability::CapabilityContext,
    channels::Receiver,
    executor::{executor_and_spawner, QueuingExecutor},
    steps::StepRegistry,
    Request, Step, WithContext,
};

/// AppTester is a simplified execution environment for Crux apps for use in
/// tests.
///
/// Create an instance of `AppTester` with your `App` and an `Effect` type
/// using [`AppTester::default`].
///
/// for example:
///
/// ```rust,ignore
/// let app = AppTester::<ExampleApp, ExampleEffect>::default();
/// ```
pub struct AppTester<App, Ef>
where
    App: crate::App,
{
    app: App,
    capabilities: App::Capabilities,
    context: Rc<AppContext<Ef, App::Event>>,
}

struct AppContext<Ef, Ev> {
    commands: Receiver<Step<Ef>>,
    events: Receiver<Ev>,
    executor: QueuingExecutor,
    steps: StepRegistry,
}

impl<App, Ef> AppTester<App, Ef>
where
    App: crate::App,
{
    /// Run the app's `update` function with an event and a model state
    ///
    /// You can use the resulting [`Update`] to inspect the effects which were requested
    /// and potential further events dispatched by capabilities.
    pub fn update(&self, event: App::Event, model: &mut App::Model) -> Update<Ef, App::Event> {
        self.app.update(event, model, &self.capabilities);
        self.context.updates()
    }

    /// Run the app's `view` function with a model state
    pub fn view(&self, model: &App::Model) -> App::ViewModel {
        self.app.view(model)
    }
}

impl<App, Ef> Default for AppTester<App, Ef>
where
    App: crate::App,
    App::Capabilities: WithContext<App, Ef>,
    App::Event: Send,
    Ef: Send + 'static,
{
    fn default() -> Self {
        let (command_sender, commands) = crate::channels::channel();
        let (event_sender, events) = crate::channels::channel();
        let (executor, spawner) = executor_and_spawner();
        let capability_context = CapabilityContext::new(command_sender, event_sender, spawner);

        Self {
            app: App::default(),
            capabilities: App::Capabilities::new_with_context(capability_context),
            context: Rc::new(AppContext {
                commands,
                events,
                executor,
                steps: StepRegistry::default(),
            }),
        }
    }
}

impl<App, Ef> AsRef<App::Capabilities> for AppTester<App, Ef>
where
    App: crate::App,
{
    fn as_ref(&self) -> &App::Capabilities {
        &self.capabilities
    }
}

impl<Ef, Ev> AppContext<Ef, Ev> {
    pub fn updates(self: &Rc<Self>) -> Update<Ef, Ev> {
        self.executor.run_all();
        let effects = self
            .commands
            .drain()
            .map(|cmd| {
                let request = self.steps.register(cmd);
                TestEffect {
                    request,
                    context: Rc::clone(self),
                }
            })
            .collect();

        let events = self.events.drain().collect();

        Update { effects, events }
    }
}

/// Update test helper holds the result of running an app update using [`AppTester::update`].
#[derive(Debug)]
pub struct Update<Ef, Ev> {
    /// Effects requested from the update run
    pub effects: Vec<TestEffect<Ef, Ev, Ef>>,
    /// Events dispatched from the update run
    pub events: Vec<Ev>,
}

impl<Ef, Ev> Update<Ef, Ev> {
    pub fn filter_effects<'a, F>(&'a self, f: F) -> impl Iterator<Item = &TestEffect<Ef, Ev, Ef>>
    where
        F: Fn(&Ef) -> bool + 'a,
    {
        self.effects.iter().filter(move |ef| f(ef.as_ref()))
    }

    pub fn filter_map_effects<'a, F, NewT>(
        &'a self,
        f: F,
    ) -> impl Iterator<Item = TestEffect<Ef, Ev, NewT>> + 'a
    where
        F: Fn(&'a Ef) -> Option<NewT> + 'a,
    {
        self.effects.iter().filter_map(move |ef| {
            Some(TestEffect {
                context: Rc::clone(&ef.context),
                request: Request {
                    uuid: ef.request.uuid.clone(),
                    effect: f(&ef.request.effect)?,
                },
            })
        })
    }
}

pub struct TestEffect<Ef, Ev, T> {
    request: Request<T>,
    context: Rc<AppContext<Ef, Ev>>,
}

impl<Ef, Ev, T> TestEffect<Ef, Ev, T> {
    pub fn map<F, NewT>(&self, f: F) -> TestEffect<Ef, Ev, NewT>
    where
        F: FnOnce(&T) -> NewT,
    {
        TestEffect {
            context: Rc::clone(&self.context),
            request: Request {
                uuid: self.request.uuid.clone(),
                effect: f(&self.request.effect),
            },
        }
    }

    pub fn resolve<Result>(&self, result: &Result) -> Update<Ef, Ev>
    where
        Result: serde::ser::Serialize,
    {
        self.context.steps.resume(
            self.request.uuid.as_slice(),
            &bcs::to_bytes(result).unwrap(),
        );
        self.context.updates()
    }
}

impl<Ef, Ev, T> AsRef<T> for TestEffect<Ef, Ev, T> {
    fn as_ref(&self) -> &T {
        &self.request.effect
    }
}

impl<Ef, Ev, T> PartialEq<T> for TestEffect<Ef, Ev, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.request.effect == *other
    }
}

impl<Ef, Ev, T> fmt::Debug for TestEffect<Ef, Ev, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestEffect")
            .field("request", &self.request)
            .finish()
    }
}
