#![feature(proc_macro_hygiene, decl_macro)]

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket::get;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;

macro_rules! safe_unwrap {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(x) => {
                error!("{:?}", x);
                return Redirect::to("/assets/unknown.gif");
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
fn fetch_badge(
    vcs: String,
    username: String,
    project: String,
    params: Option<Form<Params>>,
) -> Redirect {
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

    let resps: Vec<Resp> = safe_unwrap!(safe_unwrap!(reqwest::get(&url)).json());

    debug!("resps: {:#?}", resps);

    if resps.len() == 0 {
        return Redirect::to("/assets/unknown.gif");
    }

    Redirect::to(format!("/assets/{}.gif", resps[0].status))
}

fn main() {
    env_logger::init();
    rocket::ignite()
        .mount("/assets", StaticFiles::from("assets"))
        .mount("/", routes![fetch_badge])
        .launch();
}
