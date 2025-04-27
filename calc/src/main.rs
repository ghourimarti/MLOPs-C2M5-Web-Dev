// web microservice for calculing multiple types of calculations

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("This is a calculator microservice")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("{} + {} = {}", info.0, info.1, result))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(format!("{} - {} = {}", info.0, info.1, result))
}
#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(format!("{} * {} = {}", info.0, info.1, result))
}
#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::divide(info.0, info.1);
    match result {
        Ok(res) => HttpResponse::Ok().body(format!("{} / {} = {}", info.0, info.1, res)),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
        //.service(calculate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[post("/calculate")]
// async fn calculate(req_body: String) -> impl Responder {
//     // Here you would parse the request body and perform calculations
//     // For now, we just return the received body
//     HttpResponse::Ok().body(format!("Received: {}", req_body))
// }
