use leptos::*;
use tauri_sys::tauri::invoke;

use super::Empty;


#[component]
pub fn ShiftIdentity(cx : Scope) -> impl IntoView{
    let shift = create_resource(cx, || (), |_| async move {
        let Ok((order,(day,month,year))) = invoke::<Empty,(String,(String,String,String))>("current_shift", &Empty).await else {
            return None;
        };
        let date = day + " / " + &month + " / " + &year;
        Some((order,date))
    });

    let reader = move || match shift.read(cx) {
        Some(Some(value)) => Some(value),
        _ => None
    };

    let order = move || reader().map(|x| x.0);
    let date = move || reader().map(|x| x.1);

    view! {cx,
        <section class="SHIFT_IDENTITY">
            <div>
                <span>"التاريخ"</span>" : "<span>{date}</span>
            </div>
            <div>
                <span>"الوردية"</span>" : "<span>{order}</span>
            </div>
        </section>
    }
}
