use crate::pages::home::query::MyQuery;
use cynic::{GraphQlResponse, QueryBuilder};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

mod query;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomError {
    pub message: String,
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "can't fetch")
    }
}
impl std::error::Error for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError {
            message: format!("Reqwest error: {}", error),
        }
    }
}
pub async fn my_query(_var: i32) -> Result<Option<MyQuery>, CustomError> {
    let operation = MyQuery::build(());
    let res = reqwest::Client::new();
    let response_body = res
        .post("https://api.thegraph.com/subgraphs/name/decentraland/marketplace")
        .json(&operation)
        .send()
        .await;
    let result = response_body?.json::<GraphQlResponse<MyQuery>>().await?;
    Ok(result.data)
}
#[component]
pub fn HomePage() -> impl IntoView {
    let (var, _set_var) = create_signal(0);
    let test = create_resource(var, my_query);
    view! {
      <>
        <h1>Home Page</h1>
        <Suspense fallback=move || view! { <p>loading ...</p> }>
          <ErrorBoundary fallback=|_errors| {
              view! { <p>something went wrong</p> }
          }>
            {move || {
                test.get()
                    .map(|data| match data {
                        Ok(None) => {
                            view! {
                              <div>
                                <p>Nothing to show</p>
                              </div>
                            }
                        }
                        Ok(Some(_test)) => {
                            view! {
                              <div>
                                <p>it works just fine so fucking easy</p>
                              </div>
                            }
                        }
                        Err(e) => {
                            view! {
                              <div class="flex flex-col gap-4">
                                <p>Not supposed to happend it have an error boundary</p>
                                <p class="text-2xl text-red-600">{e.message}</p>
                              </div>
                            }
                        }
                    })
            }}

          </ErrorBoundary>
        </Suspense>
      </>
    }
}
