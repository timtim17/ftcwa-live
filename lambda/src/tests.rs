use rocket::{http::Status, local::blocking::Client};
use url::Url;

use super::build_rocket;
use super::data::{ALL_STREAMS, STREAMS};

#[test]
fn router_routes_invalid_events() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let req = client.get("/invalid");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn router_routes_valid_events() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    for &event_key in STREAMS.keys() {
        let req = client.get(format!("/{}", event_key));
        let response = req.dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some(STREAMS[event_key]));
    }
}

#[test]
fn router_routes_valid_events_for_old_years() {
    let rocket = build_rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let req = client.get(format!("/{}", event_key));
            let response = req.dispatch();
            assert_eq!(response.status(), Status::SeeOther);
            assert_eq!(response.headers().get_one("Location"), Some(streams[event_key]));
        }
    }
}

#[test]
fn every_url_is_valid() {
    for &year in ALL_STREAMS.keys() {
        let streams = &ALL_STREAMS[year];
        for &event_key in streams.keys() {
            let url = streams[event_key];
            Url::parse(url).expect(format!("event '{}' should have a valid URL", event_key).as_str());
        }
    }
    for &event_key in STREAMS.keys() {
        let url = STREAMS[event_key];
        Url::parse(url).expect(format!("event '{}' should have a valid URL", event_key).as_str());
    }
}
