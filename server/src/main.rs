use actix_web::{App, HttpServer};
use actix_files;

const PORT: &str = "localhost:3000";
const FILE: &str = "index.html";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (chapter_number, project_name) = (args.get(1).unwrap(), args.get(2).unwrap());
    let path = format!("../{}/{}", chapter_number, project_name);

    println!("serve {}/{}", path, FILE);

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/", path.clone()).index_file(FILE))
    })
    .bind(PORT)?
    .run()
}
