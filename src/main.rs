use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Resepti API")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct User {
    username: String
}

#[post("/user")]
async fn create(user: web::Json<User>) -> impl Responder {
	HttpResponse::Ok().body(format!("Welcome {}!", user.username))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
			.service(echo)
            .service(create)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}