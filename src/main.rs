use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::fs::File;
use std::io::{self, Write};

async fn index(req: HttpRequest) -> Result<NamedFile> {
    // let path: PathBuf = req.match_info().query("index").parse().unwrap();
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(NamedFile::from_file(file, "bar.txt")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
