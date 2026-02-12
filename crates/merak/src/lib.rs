pub mod common;
pub mod models;
pub mod routes;
pub mod services;

use common::code::combine_codes;

combine_codes!(
    BusinessCodes,
    [common::code::CommonCode, services::code::AuthCode,]
);
