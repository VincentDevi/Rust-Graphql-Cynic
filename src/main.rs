mod routes;
mod pages;

use leptos::*;
use routes::*;

fn main() {
    mount_to_body(||view! { <AppRouter/> })
}
