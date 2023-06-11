use leptos::*;
use leptos_router::*;

#[component]
pub fn CurrentShift(cx : Scope) -> impl IntoView {

    view! {cx,
           <h1>"Current Shift"</h1>
           <Outlet/>
    }
}
