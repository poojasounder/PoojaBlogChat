use yew::prelude::*;

use crate::components::login_form::LoginForm;
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
        <div class="login-container">
            <LoginForm />
        </div>
        </>
    }
}