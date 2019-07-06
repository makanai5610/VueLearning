use actix_web::{App, HttpServer};
use actix_files;

const PORT: &str = "localhost:3000";

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                actix_files::Files::new("/", "../static/").index_file("index.html"),
            )
    })
    .bind(PORT)?
    .run()
}
