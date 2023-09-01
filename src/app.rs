use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| 1);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state * 2))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state / 2))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
        </>
    }
}