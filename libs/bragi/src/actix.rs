use actix_web::{http, server, App, Query, Result, Json, HttpRequest, FromRequest};
use model;
use query;
use structopt::StructOpt;
use std::sync::Arc;

// TODO: pretty errors, async es

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Params {
    q: String,
}

fn autocomplete(req: HttpRequest<Arc<::Args>>) -> Result<Json<model::v1::AutocompleteResponse>> {
    let params = <Query<Params>>::extract(&req)?;
    let res = query::autocomplete(
        &params.q,
        &[],
        true,
        0,
        10,
        None,
        &req.state().connection_string,
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
                .route("/v1/autocomplete", http::Method::GET, autocomplete)
        })
        .bind(&args.bind)
        .unwrap()
        .run();
}
