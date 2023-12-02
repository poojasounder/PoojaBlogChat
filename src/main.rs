use yew::prelude::*;
mod pages;

use pages::home::Home;
use pages::not_found::NotFound;
use yew_router::prelude::*;

//Create the main app that will load all other Components
pub struct App {
    navbar_active: bool,
}

//Message enum that is used for managing the life cycle of Components
pub enum Msg {
    ToggleNavbar,
}
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {

            html! {
                <>
                <h1>{"hello"}</h1>
                <Home /> 
                </>
            }
        }
        _ => {
            html! { <NotFound /> }
        }
    }
}

//Implement the Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();

    //Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //Creates The HTML that will show up in the browser.
        html! {
            <BrowserRouter>
            <div class="background">
            {self.view_nav(&_ctx)}
            <div class="justify-content-center m-5">
                <div class="container-fluid g-0" style="height:108px;"/>
            </div>
            <main>
            <Switch<Route> render={switch} />
            </main>
            </div>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if !navbar_active {
            "collapse navbar-collapse"
        } else {
            "navbar-collapse collapse show"
        };
        html! {
            <nav class="navbar navbar-expand-lg p-2 sticky-top navbar-dark bg-dark">

                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                    {"Rust Website"}
                </Link<Route>>

                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"
                    onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                >
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class={classes!(active_class)} id="navbarSupportedContent">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                                <Link<Route> classes={classes!("nav-link")} to={Route::Home}>
                                    { "Home" }
                                </Link<Route>>
                        </li>

                        <li class="nav-item">
                            <a href="https://github.com/nmharmon8/Rust_Rocket_Yew_Tutorial" class="nav-link">
                            {"Blog"}
                            </a>
                        </li>

                        <li class="nav-item">
                            <a href="https://theadventuresofaliceandbob.com/" class="nav-link">
                            {"Login"}
                            </a>
                        </li>

                    </ul>

                </div>
            </nav>
        }
    }
}

fn main() {
    //Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    //Start the Yew framework
    yew::Renderer::<App>::new().render();
}