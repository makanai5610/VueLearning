use actix_web::{App, HttpServer};
use actix_files;

const PORT: &str = "localhost:3000";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = format!("../{}/{}", args.get(1).unwrap(), args.get(2).unwrap());

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/", path.clone()).index_file("index.html"))
    })
    .bind(PORT)?
    .run()
}
