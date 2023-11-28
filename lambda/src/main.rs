#[macro_use] extern crate rocket;

use lambda_web::{launch_rocket_on_lambda, LambdaError};
use rocket::{Build, Rocket, http::Status, response::Redirect};
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/brattain")]
fn brattain() -> Redirect {
    Redirect::temporary("https://www.youtube.com/watch?v=oD5AYrQbEsY")
}

#[get("/<paths..>")]
fn redirect(paths: PathBuf) -> Result<Redirect, Status> {
    let path = paths.as_path().to_str().unwrap();
    let path_lower = path.to_lowercase();
    if path == path.to_lowercase() {
        Err(Status::NotFound)
    } else {
        Ok(Redirect::to(format!("/{}", path_lower)))
    }
}

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    launch_rocket_on_lambda(build_rocket()).await?;
    Ok(())
}

fn build_rocket() -> Rocket<Build> {
    let rocket = rocket::build().mount("/", routes![index, redirect, brattain]);
    rocket
}
