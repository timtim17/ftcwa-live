#[macro_use] extern crate rocket;

pub mod data;

#[cfg(feature = "lambda")] use lambda_web::{launch_rocket_on_lambda, LambdaError};
use rocket::{Build, Rocket, http::Status, response::Redirect};

use crate::data::STREAMS;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<path>")]
fn events(path: &str) -> Result<Redirect, Status> {
    let path_lower = path.to_lowercase();
    match STREAMS.get(path_lower.as_str()) {
        None => Err(Status::NotFound),
        Some(url) => {
            let url: &'static str = (**url).as_ref();
            Ok(Redirect::to(url))
        },
    }
}

#[cfg(not(feature = "lambda"))]
#[rocket::launch]
fn rocket() -> Rocket<Build> {
    build_rocket()
}

#[cfg(feature = "lambda")]
#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    launch_rocket_on_lambda(build_rocket()).await?;
    Ok(())
}

fn build_rocket() -> Rocket<Build> {
    let rocket = rocket::build().mount("/", routes![index, events]);
    rocket
}
