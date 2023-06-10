use leptos::*;
use tauri_sys::tauri::invoke;

use crate::{app::Empty, shared::fun::alert};

#[component]
pub fn Logout(cx : Scope) -> impl IntoView{

    let logout = |_| spawn_local(async move {
        let res = invoke::<Empty,()>("logout", &Empty).await;
        if let Err(_) = res {
            alert("حدث خطأ اثناء تسجيل الخروج")
        }
    });

    view!{cx,
      <div class="LOGOUT">
        <button
            on:click=logout
        >"تسجيل خروج"</button>
      </div>
    }
}
