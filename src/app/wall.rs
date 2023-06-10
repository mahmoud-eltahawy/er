mod about;

use about::About;

use leptos::*;


#[component]
pub fn Wall(cx : Scope) -> impl IntoView{

    view!{cx,
          <section>
            <About/>
          </section>
    }
}
