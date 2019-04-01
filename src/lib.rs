#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use tera::Context;
use rocket_contrib::templates::Template;
use rocket::http::RawStr;
use std::path::PathBuf;
use std::path::Path;
use rocket::response::NamedFile;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn root() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("root", &context)
}

// Serve static files (e.g. css or js)
#[get("/static/<file..>")]
fn file(file:PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

pub fn online() -> () {
    rocket::ignite()
        .mount("/", routes![root, file])
        .mount("/static", StaticFiles::from("/static"))
        .attach(Template::fairing())
        .launch();
}

