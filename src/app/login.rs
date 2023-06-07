use leptos::{component, IntoView, Scope, view};
use leptos::leptos_dom::ev::SubmitEvent;




#[component]
pub fn Login(cx : Scope) -> impl IntoView{

    let handle_subbmit = move |ev : SubmitEvent| {
        ev.prevent_default();
    };

    view! {cx,
        <section>
            <form on:subbmit=handle_subbmit>
                <input
                    type="number"
                    placeholder="رقم التعريف"
                    required
                />
                <input
                    type="password"
                    placeholder="كلمة السر"
                    required
                />
                <button type="subbmit"></button>
            </form>
        </section>
    }
}
