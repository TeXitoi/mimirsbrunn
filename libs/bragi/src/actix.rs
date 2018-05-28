use actix_web::{http, server, App, Query, State, Result, Json};
use model;
use query;
use structopt::StructOpt;
use std::sync::Arc;

// TODO: pretty errors, async es

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Params {
    q: String,
}

fn autocomplete(params: Query<Params>, state: State<Arc<::Args>>) -> Result<Json<model::v1::AutocompleteResponse>> {
    let res = query::autocomplete(
        &params.q,
        &[],
        true,
        0,
        10,
        None,
        &state.connection_string,
        None,
        &[]
    );
    Ok(Json(res.into()))
}

pub fn runserver() {
    let args = Arc::new(::Args::from_args());
    let args_move = args.clone();
    server::new(
        move || {
            App::with_state(args_move.clone())
                .resource("/v1/autocomplete", |r| r.method(http::Method::GET).with2(autocomplete))
        })
        .bind(&args.bind)
        .unwrap()
        .run();
}
