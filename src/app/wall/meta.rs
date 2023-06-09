use leptos::*;

mod about;
mod logout;
mod password;

use about::About;
use logout::Logout;
use password::Password;

#[component]
pub fn Meta(cx:Scope) -> impl IntoView{
    let (password,set_password) = create_signal(cx, false);

    view!{cx,
            <About set_password=set_password/>
            <Logout/>
            <Password read_password=password set_password=set_password/>
    }
}
