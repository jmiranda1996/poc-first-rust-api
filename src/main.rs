use warp::{Filter, Rejection};

mod handler;
mod error;

type Result<T> = std::result::Result<T, Rejection>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() -> Result<()> {

    let api = warp::path("api");

    let api_routes = api
            .and(warp::get())
            .and(warp::path("info"))
            .and_then(handler::get);

    let routes = api_routes.recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}