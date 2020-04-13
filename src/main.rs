use actix_web::{post, get, App, HttpServer, HttpResponse, Error, web};
use serde::Deserialize;
use listenfd::ListenFd;
use bson::doc;
use mongodb::{Client,Collection};
use serde_json::{json, Value};


#[derive(Deserialize)]
struct FormData {
    user_name: String,
    password: String
}
struct SignupFormData {
    user_name: String,
    password: String,
    email: String
}
const client:Client= Client::with_uri_str("mongodb+srv://author:author123@cluster0-geoiq.mongodb.net/test?retryWrites=true&w=majority").expect("Failed to connect");


#[post("/signup")]
async fn index3(params: web::Form<SignupFormData>,db:Collection)->Result<HttpResponse,Error>{
    println!("name is {} \npass is : {}",params.user_name,params.password);
    let user_name = params.user_name;
    let password = params.password;
    let email = params.email;
    let docs = doc! { "user_name": user_name, "password": password, "email":email };
    let data = db.insert_one(docs, None).expect("Unable to record data");
    match data{
       Some(_)=>{
     Ok(HttpResponse::Ok().body("Account created successfully"))
        },      
       None=>{
          Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Unable to create Acoount")) 
        }
    }
}

#[post("/login")]
async fn index(params: web::Form<FormData>,db:Collection)->Result<HttpResponse,Error>{
    println!("name is {} \npass is : {}",params.user_name,params.password);
    let user_name = params.user_name;
    let password = params.password;
    let docs = doc! { "user_name": user_name, "password": password };
    let data = db.find_one(docs, None).expect("No record found");
    match data{
       Some(_)=>{
     Ok(HttpResponse::Ok().body("login Successfull"))
        },      
       None=>{
          Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../forms/login_fail.html")))
        }
    }
}

#[get("/")]
async fn index1()-> Result<HttpResponse,Error>{
     Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../forms/index.html")))
}

#[get("/signup")]
async fn index2()-> Result<HttpResponse,Error>{
     Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../forms/signup.html")))
}

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    let db = client.database("test").collection("user_details");
    let mut listenfd = ListenFd::from_env();  
    let mut  server =  HttpServer::new(|| {
            App::new().service(index(db))
            .service(index1)
            .service(index2)
            .service(index3)
        });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3050")?
    };

    server.run().await  
}
