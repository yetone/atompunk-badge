#![feature(proc_macro_hygiene, decl_macro)]

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io;

use rocket::get;
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::Response;

macro_rules! safe_unwrap {
    ($e:expr, $resp_builder:expr) => {
        match $e {
            Ok(x) => x,
            Err(x) => {
                error!("{:?}", x);
                return Ok($resp_builder.sized_body(File::open("assets/unknown.gif")?).finalize());
            }
        }
    };
}

#[derive(FromForm)]
struct Params {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
    status: String,
}

#[get("/<vcs>/<username>/<project>?<params..>")]
fn fetch_badge<'a>(
    vcs: String,
    username: String,
    project: String,
    params: Option<Form<Params>>,
) -> io::Result<Response<'a>> {
    let url = if let Some(form) = params {
        format!(
            "https://circleci.com/api/v1.1/project/{}/{}/{}?circle-token={}",
            vcs, username, project, form.token
        )
    } else {
        format!(
            "https://circleci.com/api/v1.1/project/{}/{}/{}",
            vcs, username, project
        )
    };

    debug!("url: {}", url);

    let mut resp_builder = Response::build();

    let resp_builder = resp_builder
        .header(ContentType::GIF)
        .raw_header("Cache-Control", "max-age=0, no-cache")
        .raw_header("Pragma", "no-cache");

    let resps: Vec<Resp> = safe_unwrap!(
        safe_unwrap!(reqwest::get(&url), resp_builder).json(),
        resp_builder
    );

    debug!("resps: {:#?}", resps);

    let resp_builder = if resps.is_empty() {
        resp_builder.sized_body(File::open("assets/unknown.gif")?)
    } else {
        resp_builder.sized_body(File::open(&format!("assets/{}.gif", resps[0].status))?)
    };

    Ok(resp_builder.finalize())
}

fn main() {
    env_logger::init();
    rocket::ignite().mount("/", routes![fetch_badge]).launch();
}
