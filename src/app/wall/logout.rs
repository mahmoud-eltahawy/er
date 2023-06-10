use leptos::*;
use tauri_sys::tauri::invoke;

use crate::app::Empty;


#[component]
pub fn Logout(cx : Scope) -> impl IntoView{

    let logout = |_| spawn_local(async move {
        let _ = invoke::<Empty,()>("logout", &Empty).await;
    });

    view!{cx,
      <div class="LOGOUT">
        <button
            on:click=logout
        >"تسجيل خروج"</button>
      </div>
    }
}
