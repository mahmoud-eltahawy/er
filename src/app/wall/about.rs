use leptos::*;
use models::{employee::{Employee, Position}, department::Department};
use tauri_sys::tauri::invoke;

use crate::app::Id;

#[component]
pub fn About(cx : Scope) -> impl IntoView {
    let employee_signal = use_context::<ReadSignal<Option<Employee>>>(cx);
    let employee = move || employee_signal
        .expect("employee signal must be provided")
        .get()
        .expect("employee must be some");


    let department = create_resource(cx,
            move || employee().department_id,
            |id| async move {
            invoke::<Id,Department>("find_department", &Id{id}).await.unwrap()
    });

    let position = move || {
        let is_boss = match department.read(cx) {
            Some(department) => {
                match department.boss_id {
                    Some(boss_id) => {
                        Some(if employee().id == boss_id
                        {" و رئيس القسم"}
                        else {""})
                    },
                    None => {
                        log!("department {} does not have boss",department.name);
                        None
                    }
                }
            },
            None => {
                log!("no department assigned to the employee");
                None
            }
        };
        let superiority = if employee().position == Position::SuperUser
        { "مشرف" }
        else { "مستخدم" };

        match is_boss {
            Some(is_boss) => {
                superiority.to_string() + is_boss
            },
            None => superiority.to_string()
        }
    };

    view!{cx,
        <section class="ABOUT">
          <Show
            when=move || employee().card_id != 0
            fallback=|cx| view! {cx,<p>"الحساب الرئيسي"</p>}
          >
            <p>
                <span>"الاسم : "</span>
                <span>{move || employee().first_name}</span>
                <span>" "</span>
                <span>{move || employee().middle_name}</span>
                <span>" "</span>
                <span>{move || employee().last_name}</span>
            </p>
            <p class="about-in">
                <span>"الرقم التعريفي : "</span>
                <span>{move || employee().card_id}</span>
            </p>
            <Show
            fallback=|cx| view! {cx,<p>"جاري تحميل القسم..."</p>}
            when=move || matches!(department.read(cx),Some(_))
            >
                <p class="about-in">
                    <span>"القسم : "</span>
                    <span>{move || department.read(cx).unwrap().name}</span>
                </p>
                <p class="about-in">
                    <span>"الرتبة : "</span>
                    <span>{position}</span>
                </p>
            </Show>
          </Show>
          <button class="about-in" on:click=|_| log!("later")>"تغيير كلمة السر"</button>
        </section>
    }
}
