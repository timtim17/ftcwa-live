#[macro_use] extern crate rocket;

pub mod data;

use either::{Either, Left, Right};
#[cfg(feature = "lambda")] use lambda_web::{launch_rocket_on_lambda, LambdaError};
use rocket::{http::Status, response::{content::RawHtml, Redirect}, Build, Rocket};

use crate::data::{ALL_STREAMS, STREAMS, STATIC_PAGES};

#[get("/")]
fn index() -> &'static str {
    "Usage: https://ftcwa.live/<event name>"
}

#[get("/<year>/<path>")]
fn events_specific_year(year: &str, path: &str) -> Result<Either<RawHtml<&'static str>, Redirect>, Status> {
    let path_lower = path.to_lowercase();
    match ALL_STREAMS.get(year) {
        None => match STATIC_PAGES.get(format!("{}/{}", year, path_lower).as_str()) {
            None => Err(Status::NotFound),
            Some(&content) => Ok(Left(RawHtml(content))),
        },
        Some(streams) => match streams.get(path_lower.as_str()) {
            None => Err(Status::NotFound),
            Some(&url) => Ok(Right(Redirect::to(url))),
        },
    }
}

#[get("/<path>")]
fn events(path: &str) -> Result<Either<RawHtml<&'static str>, Redirect>, Status> {
    let path_lower = path.to_lowercase();
    match STREAMS.get(path_lower.as_str()) {
        None => match STATIC_PAGES.get(path_lower.as_str()) {
            None => Err(Status::NotFound),
            Some(&content) => Ok(Left(RawHtml(content))),
        },
        Some(&url) => Ok(Right(Redirect::to(url))),
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
    rocket::build()
        .mount("/", routes![index, events, events_specific_year])
}

#[cfg(test)] mod tests;
