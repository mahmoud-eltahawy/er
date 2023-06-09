use futures::{future, StreamExt};
use leptos::*;
use models::{employee::Employee, permissions::PermissionName};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tauri_sys::{tauri::invoke,event::{listen, Event}};
use uuid::Uuid;

mod login;
mod shift_identity;

use login::Login;
use shift_identity::ShiftIdentity;

pub fn listen_to<T : DeserializeOwned + 'static>(event_name : String,action : impl Fn(Event<T>) + 'static) {
    spawn_local(async move {
        if let Ok(events) = listen(&event_name).await {
            events.for_each(|event| {
                action(event);
                future::ready(())
            }).await;
        }
    })
}

#[derive(Serialize, Deserialize)]
struct Empty;
#[derive(Serialize, Deserialize)]
struct Id{
    id : Uuid
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (employee, set_employee) = create_signal(cx, None::<Employee>);
    let (shift_id, set_shift_id) = create_signal(cx, None::<Uuid>);
    let (permissions, set_permissions) = create_signal(cx, Vec::<PermissionName>::new());

    let is_loggedin = move |ids : Option<(Uuid,Uuid)>| {
        spawn_local(async move {
            let ids = match ids {
                Some((employee_id,shift_id)) => Ok((employee_id,shift_id)),
                None => invoke::<Empty,(Uuid,Uuid)>("check_login", &Empty).await
            };
            if let Ok((employee_id,shift_id)) = ids {
                if let (Ok(employee),Ok(permissions)) = (
                    invoke::<Id,Employee>("get_employee_by_id",&Id { id: employee_id } ).await,
                    invoke::<Id,Vec<PermissionName>>("employee_permissions",&Id { id: employee_id } ).await
                ) {
                    set_employee.set(Some(employee));
                    set_shift_id.set(Some(shift_id));
                    set_permissions.set(permissions);
                };
            };
        })
    };

    is_loggedin(None);
    spawn_local(async move {
        let _ = invoke::<Empty,()>("update", &Empty).await;
    });

    let end_the_shift = move |_| {
        set_employee.set(None);
        set_shift_id.set(None);
        set_permissions.set(Vec::new());
    };

    listen_to::<()>("shift_ended".to_string(), end_the_shift);
    listen_to::<()>("logout".to_string(), end_the_shift);
    listen_to::<(Uuid,Uuid)>("new_login".to_string(), move |e| is_loggedin(Some(e.payload)));
    listen_to::<(Uuid,PermissionName)>("update_employee_allow_permission".to_string(), move |e| {
        let (id,permission) = e.payload;
        if let Some(employee) = employee.get() {
            if employee.id == id {
                set_permissions.update(|xs| xs.push(permission));
            }
        }
    });
    listen_to::<(Uuid,PermissionName)>("update_employee_forbid_permission".to_string(), move |e| {
        let (id,permission) = e.payload;
        if let Some(employee) = employee.get() {
            if employee.id == id {
                set_permissions.set(permissions.get().into_iter().filter(|x| x != &permission).collect());
            }
        }
    });
    listen_to::<Uuid>("update_employee_forbid_all_permissions".to_string(), move |e| {
        let id = e.payload;
        if let Some(employee) = employee.get() {
            if employee.id == id {
                set_permissions.set(Vec::new());
            }
        }
    });
    listen_to::<Uuid>("update_employee_up".to_string(), move |e| {
        let id = e.payload;
        if let Some(employee) = employee.get() {
            if employee.id == id {
                set_employee.set(Some(Employee{position : "SUPER_USER".to_string(),..employee}))
            }
        }
    });
    listen_to::<Uuid>("update_employee_down".to_string(), move |e| {
        let id = e.payload;
        if let Some(employee) = employee.get() {
            if employee.id == id {
                set_employee.set(Some(Employee{position : "USER".to_string(),..employee}))
            }
        }
    });

    provide_context(cx, employee);
    provide_context(cx, shift_id);
    provide_context(cx, permissions);

    view! { cx,
        <main>
            <ShiftIdentity/>
            <Show
              fallback=move |cx| view!{cx, <Login/>}
              when=move || matches!(employee.get(),Some(_))
            >
                {view!{cx, <p>{employee.get().unwrap().first_name}</p>}}
            </Show>
        </main>
    }
}
