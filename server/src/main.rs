use actix_files;
use actix_web::{App, HttpServer};
use sysinfo;

const PORT: &str = "localhost:3000";
const FILE: &str = "index.html";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (chapter_number, project_name) = match (args.get(1), args.get(2)) {
        (Some(cn), Some(pn)) => (cn, pn),
        _ => {
            println!("args is invalid");
            std::process::exit(1);
        }
    };

    match sysinfo::get_current_pid() {
        Ok(pid) => println!("[PID] {}", pid),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let path = format!("../{}/{}", chapter_number, project_name);

    println!("serve {}/{}", path, FILE);

    HttpServer::new(move || {
        App::new().service(actix_files::Files::new("/", path.clone()).index_file(FILE))
    })
    .bind(PORT)?
    .run()
}
