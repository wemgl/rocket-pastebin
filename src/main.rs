#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
use std::path::Path;

use rocket::Data;
use rocket::http::Status;

use paste_id::PasteId;

mod paste_id;

#[get("/")]
fn index() -> &'static str {
    "\
    USAGE

        POST /

            accepts raw data in the body of the request and responds with a URL of
            a page containing the body's content

        GET /<id>

            retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(3);
    let upload_path = format!("upload/{id}", id = id);

    let url = match paste.stream_to_file(&upload_path) {
        Ok(_) => format!("http://localhost:8000/{id}", id = id),
        Err(_) => Status::InternalServerError.to_string(),
    };

    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve])
        .launch();
}
