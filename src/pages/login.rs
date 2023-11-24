use yew::prelude::*;

use crate::components::input::*;
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="form-container">
            <form class="login-form">
                <Input input_type="text" name="username" label="Username"/>
                <Input input_type="password" name="password" label="Password" />
                <button class="form-button" type="submit">{"Login"}</button>
            </form>
        </div>
    }
}