use crate::pages::home::query::MyQuery;
use cynic::{GraphQlResponse, QueryBuilder};
use leptos::*;

mod query;

pub async fn my_query() {
    let operation = MyQuery::build(());
    let res = reqwest::Client::new();
    let response_body = res.get("").json(&operation).send()?;

    let result = response_body.json::<GraphQlResponse<MyQuery>>()?;
    result
}
#[component]
pub fn HomePage() -> impl IntoView {
    let (ok, set_ok) = create_signal(0);
    view! {
      <>
      <h1> Home Page </h1>

      </>
    }
}
