use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
        <div class="login-container">
            <form class ="login-form">
                <label class="form-label">{"Username"}</label>
                <input type = "text"  class="form-input"/>
                <label  class="form-label">{"Password"}</label>
                <input type="password" class="form-input"/>
                <button type="submit" class="form-button">{"Login"}</button>
            </form>
        </div>
        </>
    }
}