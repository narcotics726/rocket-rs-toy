use std::path::{Path, PathBuf};

use rocket::fs::{relative, FileServer, NamedFile};

#[rocket::get("/api/hello")]
fn r_hello() -> &'static str {
    "hello"
}

#[rocket::get("/<_..>")]
async fn index() -> Option<NamedFile> {
    let mut p = Path::new(relative!("static/index.html"));
    NamedFile::open(relative!("static/index.html")).await.ok()
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![r_hello])
        .mount("/", rocket::routes![index])
}
