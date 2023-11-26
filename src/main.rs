use yew::prelude::*;

mod pages;
mod components;
mod models;
mod schema;
mod repositories;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="background">
        <pages::login::Login />
        </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
