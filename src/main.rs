use yew::prelude::*;

mod pages;

#[function_component(App)]
fn app() -> Html {
    html! {
        <pages::login::Login />
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
