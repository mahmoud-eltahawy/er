use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx:Scope) -> impl IntoView{
    let (show,set_show) = create_signal(cx, false);

    view!{cx,
          <Show
            when=move || show.get()
            fallback=move|cx| view!{cx,<button on:click=move|_| set_show.set(true)>"nav"</button>}
          >
            <ul class="NAVBAR">
              <li><A href="/controll">"controll"</A></li>
              <li><A href="/current_shift">"current shift"</A></li>
              <li><A href="/history">"history"</A></li>
            </ul>
          </Show>
    }
}
