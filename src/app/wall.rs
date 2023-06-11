use leptos::*;
use leptos_router::*;

mod meta;
mod controll;
mod current_shift;
mod history;

use meta::Meta;
use controll::Controll;
use current_shift::CurrentShift;
use history::History;

#[component]
pub fn Wall(cx : Scope) -> impl IntoView{

    view!{cx,
        <Router>
          <Meta/>
          <section>
            <Routes>
              <Route path="/" view=|cx| view! {cx,
                    <div>
                      <A href="/controll">"controll"</A><br/>
                      <A href="/current_shift">"current shift"</A><br/>
                      <A href="/history">"history"</A><br/>
                    </div>}
              />
              <Route path="/controll" view=|cx| view! {cx,<Controll/>} />
              <Route path="/current_shift" view=|cx| view! {cx,<CurrentShift/>} />
              <Route path="/history" view=|cx| view! {cx,<History/>} />
            </Routes>
          </section>
        </Router>
    }
}
