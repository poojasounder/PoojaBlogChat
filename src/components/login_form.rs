use yew::prelude::*;

use crate::components::input::*;
#[function_component(LoginForm)]
pub fn login_form()-> Html {
    let onsubmit = Callback::from(|e: SubmitEvent|{
        e.prevent_default();
        
    });
    html!{
        <>
            <form class="login-form" onsubmit={onsubmit}>
                <Input input_type="text" name="username" label="Username"/>
                <Input input_type="password" name="password" label="Password" />
                <button class="form-button" type="submit">{"Login"}</button>
            </form>
        </>
    }
}