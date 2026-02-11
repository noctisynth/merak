pub mod auth;
pub mod common;
pub mod models;
pub mod routes;

use common::code::combine_codes;

combine_codes!(
    BusinessCodes,
    [common::code::CommonCode, auth::code::AuthCode,]
);
