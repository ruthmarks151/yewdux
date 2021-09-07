use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone)]
struct State {
    count: u32,
}

struct App {
    /// Our local version of state.
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
}

enum Msg {
    /// Message to receive new state.
    State(Rc<State>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.state.count;
        let onclick = self
            .dispatch
            .future_callback(|dispatch| async move { dispatch.reduce(|s| s.count += 1) });
        html! {
            <>
            <h1>{ count }</h1>
            <button onclick={onclick}>{"+1"}</button>
            </>
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}