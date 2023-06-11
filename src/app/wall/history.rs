use leptos::*;
use leptos_router::*;

#[component]
pub fn History(cx : Scope) -> impl IntoView {

    view! {cx,
           <h1>"History"</h1>
           <Outlet/>
    }
}
