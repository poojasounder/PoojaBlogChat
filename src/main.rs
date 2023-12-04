use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Frontend App with Rust and Yew.rs: User SignUp and Login"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
