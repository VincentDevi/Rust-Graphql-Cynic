mod routes;

use leptos::*;
use routes::*;

fn main() {
    mount_to_body(||view! {<AppRouter/>})
}
