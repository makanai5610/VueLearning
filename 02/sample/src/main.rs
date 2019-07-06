use actix_web::{App, HttpServer};
use actix_files;

const PORT: &str = "localhost:3000";

enum Path {
    Items, Lifecycle
}

impl Path {
    fn build(&self) -> String {
        format!("../static/{}", match self {
            Path::Items => "items",
            Path::Lifecycle => "lifecycle"
        })
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/", Path::Items.build()).index_file("index.html"))
    })
    .bind(PORT)?
    .run()
}
