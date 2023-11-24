use yew::prelude::*;

use crate::components::login_form::*;
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="form-container">
            <LoginForm />
        </div>
    }
}