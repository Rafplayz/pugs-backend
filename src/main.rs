#[macro_use]
extern crate rocket;
use dotenv;
use rocket::fs::FileServer;
use std::format as f;

#[get("/api/ping/<what>")]
fn index_with_params(what: &str) -> String {
  f!("ay {what}")
}

#[get("/api/ping")]
fn index() -> &'static str {
  "ay yo"
}

#[launch]
fn rocket() -> _ {
  let dir = dotenv::var("DIR").unwrap();
  let config = rocket::Config {
    port: 3000,
    ..rocket::Config::debug_default()
  };
  let static_file_server = FileServer::from(dir);
  rocket::build()
    .configure(config)
//  .attach(Cors)
    .mount("/", routes![index, index_with_params])
    .mount("/", static_file_server)
}

// // cors, please fuck off
// use rocket::{Request, Response};
// use rocket::http::Header;
// pub struct Cors;

// #[rocket::async_trait]
// impl rocket::fairing::Fairing for Cors {
//     fn info(&self) -> rocket::fairing::Info {
//         rocket::fairing::Info {
//             name: "Cross-Origin-Resource-Sharing Fairing",
//             kind: rocket::fairing::Kind::Response,
//         }
//     }

//     async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new(
//             "Access-Control-Allow-Methods",
//             "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
//         ));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//     }
// }
