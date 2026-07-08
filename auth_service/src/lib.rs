#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

// mod auth_utils {
//     pub fn login(creds: models::Credentials) {
//         // authenticate...
//         crate::database::get_user();
//     }
//     fn logout() {
//         // log user out
//     }

//     pub mod models {
//         pub struct Credentials {
//             pub username: String,
//             pub password: String,
//         }
//     }
// }

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
