mod about;
mod logout;

use about::About;
use logout::Logout;

use leptos::*;


#[component]
pub fn Wall(cx : Scope) -> impl IntoView{

    view!{cx,
          <section>
            <About/>
            <Logout/>
          </section>
    }
}
