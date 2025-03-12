use yew::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// Define the structure to hold user details.
#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
}

#[function_component(App)]
fn app() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let user_details = use_state(|| None::<User>);

    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let user_details = user_details.clone();
        Callback::from(move |_| {
            let user = User {
                name: (*name).clone(),
                email: (*email).clone(),
            };
            user_details.set(Some(user));
        })
    };

    html! {
        <div style="text-align: center; padding: 20px; font-family: Arial;">
            <h1>{"User Details Form"}</h1>
            <input type="text" placeholder="Enter Name" value={(*name).clone()} oninput={on_name_input} />
            <br /><br />
            <input type="email" placeholder="Enter Email" value={(*email).clone()} oninput={on_email_input} />
            <br /><br />
            <button onclick={on_submit}>{"Submit"}</button>
            
            <h2>{"User Information"}</h2>
            {
                if let Some(user) = (*user_details).clone() {
                    html! {
                        <div>
                            <p><strong>{"Name: "}</strong>{ user.name }</p>
                            <p><strong>{"Email: "}</strong>{ user.email }</p>
                        </div>
                    }
                } else {
                    html! { <p>{"No user details submitted yet."}</p> }
                }
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::with_root(web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("app").unwrap()
        .into())
    .render();
}
