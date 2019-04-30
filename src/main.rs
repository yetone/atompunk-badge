#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket::get;
use rocket::request::Form;
use rocket::response::NamedFile;
use std::path::Path;
use rocket_codegen::routes;

#[derive(FromForm)]
struct Params {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
    status: String,
}

#[get("/<vcs>/<username>/<project>?<params..>")]
fn fetch_badge(
    vcs: String,
    username: String,
    project: String,
    params: Option<Form<Params>>) -> Option<NamedFile> {

    let url = if let Some(form) = params {
        format!("https://circleci.com/api/v1.1/project/{}/{}/{}?circle-token={}", vcs, username, project, form.token)
    } else {
        format!("https://circleci.com/api/v1.1/project/{}/{}/{}", vcs, username, project)
    };

    println!("url: {}", url);

    let resps: Vec<Resp> = reqwest::get(&url).unwrap().json().unwrap();

    println!("resps: {:#?}", resps);

    if resps.len() == 0 {
        return None;
    }

    NamedFile::open(Path::new(&format!("assets/{}.gif", resps[0].status))).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![fetch_badge]).launch();
}
