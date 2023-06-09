use leptos::*;
use leptos::html::Input;
use leptos::leptos_dom::ev::SubmitEvent;
use tauri_sys::tauri::invoke;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct LoginArgs {
    cardid : i64,
    password : String,
}

#[component]
pub fn Login(cx : Scope) -> impl IntoView{
    let card_ref: NodeRef<Input> = create_node_ref(cx);
    let password_ref: NodeRef<Input> = create_node_ref(cx);

    let handle_submit = move |ev : SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let cardid = card_ref.get().unwrap().value().parse().unwrap();
            let password = password_ref.get().unwrap().value();
            log!("{:#?}",cardid);
            log!("{:#?}",password);
            let r = invoke::<LoginArgs,()>("login", &LoginArgs {
                cardid,
                password
            }).await;
            log!("{:#?}",r);
        });
    };

    view! {cx,
        <section class="LOGIN">
            <form on:submit=handle_submit>
                <input
                    node_ref=card_ref
                    type="number"
                    placeholder="رقم التعريف"
                    required
                />
                <input
                    node_ref=password_ref
                    type="password"
                    placeholder="كلمة السر"
                    required
                />
                <button type="submit">"تاكيد"</button>
            </form>
        </section>
    }
}
