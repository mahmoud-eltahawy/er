use leptos::*;
use leptos_router::*;

#[component]
pub fn Controll(cx : Scope) -> impl IntoView {

    view! {cx,
           <h1>"Controll"</h1>
           <Outlet/>
    }
}
