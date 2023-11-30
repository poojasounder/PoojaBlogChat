use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};
use crate::api::user::{api_login, api_me, LoginResponse, MeResponse};

use crate::components::input::*;
#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let username_changed = Callback::from(move |e:Event|{
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value());
        }

    });
    let password_changed = Callback::from(move |e:Event|{
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handle.set(input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        spawn_local(async move {
            api_login(cloned_username.clone(),cloned_password.clone()).await;
        });
    });
    html!{
        <form class ="login-form" onsubmit={onsubmit}>
                <Input 
                    input_type="text" 
                    name="username" 
                    label="Username"
                    value={username}
                    onchange={username_changed}
                />
                <Input 
                    input_type="password" 
                    name="password" 
                    label="Password"
                    value={password}
                    onchange={password_changed}
                />
                <button type="submit" class="form-button">{"Login"}</button>
            </form>
    }
}