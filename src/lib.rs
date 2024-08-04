use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std ::net :: TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
//    let server = HttpServer::new(|| {
//        App::new()
//            .route("/health_check", web::get().to(health_check))
//    })
//    .listen(listener)?
//    .run();
    // No .await here
//    Ok (server)
//}

// Let's start simple: we always return a 200 ok

async fn subscribe()-> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener:TcpListener)->Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here
    Ok (server)
}