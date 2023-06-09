use leptos::*;

use super::listen_to;

#[component]
pub fn MajorUpdate(cx : Scope) -> impl IntoView {
    let (successes_and_failures,set_successes_and_failures) = create_signal(cx, (0,0));
    let (updates_number,set_updates_number) = create_signal(cx, None::<u64>);


    listen_to::<u64>(
        "begin_major_update".to_string(),
        move |e| set_updates_number.set(Some(e.payload)));
    listen_to::<()>(
        "end_major_update".to_string(),
        move |_| set_updates_number.set(None));
    listen_to::<(u64,u64)>(
        "major_update_step".to_string(),
        move |e| set_successes_and_failures.set(e.payload));

    view!{cx,
        <Show
          when=move || match updates_number.get() {
            Some(number) if number > 50 => true,
            _ => false,
          }
          fallback=move |cx| view!{cx, <div style="display:none"></div>}
        >
          <section class="MAJOR_UPDATE">
            <h1>"تحديث بيانات"</h1>
            <p>
                <span>"البيانات المتاحة : "</span>
                <span>{updates_number.get()}</span>
            </p>
            <p>
                <span>"تمت محاولة تحديث : "</span>
                <span>{successes_and_failures.get().0 + successes_and_failures.get().1}</span>
            </p>
            <p>
                <span>"التحديثات الناجحة الي البيانات المتاحة : "</span>
                <span>{successes_and_failures.get().0}</span>
                <span>"الي"</span>
                <span>{updates_number.get()}</span>
                <span>" "</span>
                <span>{(successes_and_failures.get().0 / updates_number.get().unwrap()) * 100}</span>
                <span>" % "</span>
            </p>
            <p>
                <span>"التحديثات الفاشلة الي البيانات المتاحة : "</span>
                <span>{successes_and_failures.get().1}</span>
                <span>"الي"</span>
                <span>{updates_number.get()}</span>
                <span>" "</span>
                <span>{(successes_and_failures.get().1 / updates_number.get().unwrap()) * 100}</span>
                <span>" % "</span>
            </p>
          </section>
        </Show>
    }
}
