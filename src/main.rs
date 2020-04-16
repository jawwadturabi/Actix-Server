use actix_web::{post, get, App, HttpServer, HttpResponse, Error, web};
use serde::Deserialize;
use listenfd::ListenFd;
use bson::doc;
use std::thread;
use std::sync::mpsc;
// use std::time::Duration;
// use mongodb::db::ThreadedDatabase;
use mongodb::{options::ClientOptions,Client};
// use r2d2::Pool;
// use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};


#[derive(Deserialize)]
struct FormData {
    user_name: String,
    password: String
}
#[derive(Deserialize)]
struct SignupFormData {
    user_name: String,
    password: String,
    email: String
}


#[post("/signup")]
async fn index3(params: web::Form<SignupFormData>)->Result<HttpResponse,Error>{
    println!("name is {} \npass is : {}",params.user_name,params.password);
    let user_name = &params.user_name;
    let password = &params.password;
    let email = &params.email;
    let docs = doc! { "user_name": user_name, "password": password, "email":email };
    // let data = db.collection("user_details").insert_one(docs, None).unwrap();
    // println!("data inserted with id : {:?}",data.inserted_id);
     Ok(HttpResponse::Ok().body("Account created successfully"))

        //   Ok(HttpResponse::Ok()
        // .content_type("text/html; charset=utf-8")
        // .body("Unable to create Acoount")) 
}

#[post("/login")]
async fn index(params: web::Form<FormData>)->Result<HttpResponse,Error>{

    // let client= Client::with_uri_str("mongodb+srv://author:author123@cluster0-geoiq.mongodb.net/test?retryWrites=true&w=majority").expect("Failed to connect");
    // let db = client.database("test").collection("user_details");
    println!("name is {} \npass is : {}",params.user_name,params.password);
    let _user_name = &params.user_name;
    let _password = &params.password;

    Ok(HttpResponse::Ok().body("login Successfull"))
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
    
    //tried with thread sharing

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| {
        let client= Client::with_uri_str("mongodb+srv://author:author123@cluster0-geoiq.mongodb.net/test?retryWrites=true&w=majority").expect("Failed to connect");
    let db = client.database("test").collection("user_details");
        tx.send(db).unwrap();
});

let mut  server =  HttpServer::new(|| {
            App::new()
            .data(rx.recv().unwrap())
            .service(index)
            .service(index1)
            .service(index2)
            .service(index3)
        });
    
// tried with normal


// let mut  server =  HttpServer::new(move|| {
//             App::new()
//             .service(index)
//             .service(index1)
//             .service(index2)
//             .service(index3)
//         });


    let mut listenfd = ListenFd::from_env();  
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3050")?
    };

    server.run().await  
}
