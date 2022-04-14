use yew::prelude::*;

#[function_component(WorkInProgress)]
fn work_in_progress() -> Html {
    html! {
        <>
            <h1>{ "GodHand" }</h1>
            <p>{ "Work in progress" }</p>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <WorkInProgress />
    }
}

fn main() {
    yew::start_app::<App>();
}
