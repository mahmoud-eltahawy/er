use leptos::*;
use leptos_router::*;

mod meta;
mod controll;
mod current_shift;
mod history;
mod navbar;

use meta::Meta;
use controll::Controll;
use current_shift::CurrentShift;
use history::History;
use navbar::Navbar;

#[component]
pub fn Wall(cx : Scope) -> impl IntoView{

    view!{cx,
        <Router>
          <Meta/>
          <Navbar/>
          <section>
            <Routes>
              <Route path="/controll" view=|cx| view! {cx,<Controll/>} />
              <Route path="/current_shift" view=|cx| view! {cx,<CurrentShift/>} />
              <Route path="/history" view=|cx| view! {cx,<History/>} />
            </Routes>
          </section>
        </Router>
    }
}
