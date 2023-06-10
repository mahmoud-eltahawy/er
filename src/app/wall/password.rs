use leptos::{*, ev::SubmitEvent, html::Input};
use models::employee::Employee;
use serde::{Serialize, Deserialize};
use tauri_sys::tauri::invoke;
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
struct Args{
  employeeid : Uuid,
  oldpassword : String,
  newpassword : String,
}

#[component]
pub fn Password(
  cx: Scope,
  read_password : ReadSignal<bool>,
  set_password : WriteSignal<bool>,
) -> impl IntoView {
    let old_password_ref : NodeRef<Input> = create_node_ref(cx);
    let new_password1_ref : NodeRef<Input> = create_node_ref(cx);
    let new_password2_ref : NodeRef<Input> = create_node_ref(cx);

    let employee_signal = use_context::<ReadSignal<Option<Employee>>>(cx);
    let employee = move || employee_signal
        .expect("employee signal must be provided")
        .get()
        .expect("employee must be some");

    let handle_submit = move |ev : SubmitEvent| {
      ev.prevent_default();
      if new_password1_ref.get().unwrap().value() == new_password2_ref.get().unwrap().value() {
        spawn_local(async move {
          let res = invoke::<Args,()>("change_password", &Args{
            employeeid : employee().id,
            oldpassword : old_password_ref.get().unwrap().value(),
            newpassword : new_password1_ref.get().unwrap().value(),
          }).await;
          match res {
            Ok(_) => set_password.set(false),
            Err(err) => log!("{}",err.to_string())
          }
        })
      }
    };

    view!{cx,
        <Show
          when=move || read_password.get()
          fallback=|cx| view! {cx,<div></div>}
        >
            <section class="PASSWORD_UPDATE">
              <form on:submit=handle_submit>
                <input
                  node_ref=old_password_ref
                  type="password"
                  placeholder="كلمة السر الحالية"
                />
                <br />
                <input
                  node_ref=new_password1_ref
                  type="password"
                  placeholder="كلمة السر الجديدة"
                />
                <br />
                <input
                  node_ref=new_password2_ref
                  type="password"
                  placeholder="كلمة السر الجديدة مرة اخري"
                />
                <br />
                <button type="submit">"تاكيد"</button>
                <button
                  type="reset"
                  on:click=move |_| set_password.set(false)
                >
                  "الغاء"
                </button>
              </form>
            </section>
          </Show>
    }
}
