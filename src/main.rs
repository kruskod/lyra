// mod json;

extern crate eposlib;
#[macro_use]
extern crate rocket;

use std::{env, io};

use eposlib::config::{ElisionInput, ParserInput};
use eposlib::config::Config;
use eposlib::lm::LanguageModel;
// use std::fs::File;
use eposlib::lm;
use rocket::serde::json::{Json, json, Value};
// use rocket::serde::json::{Json, json, Value, serde_json};
use rocket::State;
use eposlib::cky::ParserOutput;
use rocket::response::status::NotFound;
use std::sync::Arc;
use rocket::tokio::task::spawn_blocking;


#[get("/parse", format = "json", data = "<p>")]
async fn parse(mut p: Json<ParserInput>, lm: &State<Arc<LanguageModel>>) -> Result<Json<Vec<ParserOutput>>, NotFound<String>> {
    // serde_json::to_string(&point).unwrap()
    info!("{}", json!(&*p));

    let lm_inner = lm.inner().clone();

    let parses = spawn_blocking(move || eposlib::parse_standard(p.swap_words(), &p.tags, lm_inner, p.num, p.pretty)).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e)).unwrap();

    match parses {
        Ok(parses) => { Ok(Json(parses)) }
        Err(e) => {
            Err(NotFound(e))
        }
    }

    // match eposlib::parse_standard(p.swap_words(), &p.tags, &lm, p.num, p.pretty) {
    //     Ok(parses) => { Ok(Json(parses)) }
    //     Err(e) => {
    //         Err(NotFound(e))
    //     }
    // }
}

//noinspection RsMainFunctionNotFound
#[get("/elision", format = "json", data = "<e>")]
async fn elision(mut e: Json<ElisionInput>, lm: &State<Arc<LanguageModel>>) -> Result<Json<Vec<ParserOutput>>, NotFound<String>> {
    info!("{}", json!(&*e));
    // info!("{}", serde_json::to_string_pretty(&*e).unwrap());

    let lm_inner = lm.inner().clone();

    let parses = spawn_blocking(move || eposlib::parse_ellipsis(e.query.swap_words(), &e.query.tags, lm_inner, e.query.num, e.query.pretty, &e.amendments)).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e)).unwrap();

    match parses {
        Ok(parses) => { Ok(Json(parses)) }
        Err(e) => {
            Err(NotFound(e))
        }
    }

    // match eposlib::parse_ellipsis(e.query.swap_words(), &e.query.tags, lm_inner, e.query.num, e.query.pretty, &e.amendments) {
    //     Ok(parses) => { Ok(Json(parses)) }
    //     Err(e) => {
    //         Err(NotFound(e))
    //     }
    // }
}

// #[get("/parse", format = "json", data = "<p>")]
// async fn parse(p: Json<ParserInput>, lm: &State<Arc<LanguageModel>>) -> Result<Json<Vec<ParserOutput>>, NotFound<String>> {
//     let calc = spawn_blocking(move || eposlib::parse_standard(&p.words, &p.tags, Arc::clone(&lm), p.num, p.pretty))
//         .await;
//
//     match calc {
//         Ok(parses) => {
//             match parses {
//                 Ok(parses) => { Ok(Json(parses)) }
//                 Err(e) => {
//                     Err(NotFound(e))
//                 }
//             }
//         }
//         Err(e) => { Err(NotFound(e.to_string())) }
//     }
//
//     // Ok(Json(parses))
//     // match parses {
//     //     Ok(parses) => { Ok(Json(parses)) }
//     //     Err(e) => {
//     //         Err(NotFound(e))
//     //     }
//     // }
// }

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    let config = Config::new(env::args()).unwrap();
    // let lm = lm::load_model(&config).unwrap();

    rocket::build()
        .manage(Arc::new(lm::load_model(&config).unwrap()))
        .mount("/", routes![parse, elision])
        .register("/", catchers![not_found])
    // .launch()
    // .await;
    // rocket::build().attach(parse)
    // rocket::build()
    //     .mount("/", routes![parse])
    //     .register("/", catchers![not_found])
}

// #[rocket::main]
// async fn main() {
//     let config = Config::new(env::args()).unwrap();
//     // let lm = lm::load_model(&config).unwrap();
//
//     rocket::build()
//         .manage(lm::load_model(&config).unwrap())
//         .mount("/", routes![parse])
//         .register("/", catchers![not_found])
//         .launch()
//         .await;
// }

// use std::io;
//
// use rocket::tokio::task::spawn_blocking;
//
// #[get("/blocking_task")]
// async fn blocking_task() -> io::Result<Vec<u8>> {
//     // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
//     let vec = spawn_blocking(|| std::fs::read("data.txt")).await
//         .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
//
//     Ok(vec)
// }