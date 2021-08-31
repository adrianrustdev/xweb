mod helpers;
mod routes;
use actix_web::{App, HttpServer};

#[cfg(not(target_arch = "wasm32"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::index))
        .bind("0.0.0.0:8040")?
        .run()
        .await
}