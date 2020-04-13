use actix_web::{post, get, App, HttpServer, HttpResponse, Error, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    user_name: String,
    password: String
}

#[post("/login")]
async fn index(params: web::Form<FormData>)->Result<HttpResponse,Error>{
    println!("name is {} \npass is : {}",params.user_name,params.password);
    Ok(HttpResponse::Ok().body("login successful"))
}
#[get("/")]
async fn index1()-> Result<HttpResponse,Error>{
     Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./index.html")))
}

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| {
            App::new().service(index)
            .service(index1)

    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
