use rocket::Route;

pub mod error;
pub mod image;

/// Creates all /api/v1/ routes for initialization
pub fn create_v1_routes() -> Vec<Route> {
    rocket::routes![image::upload::upload]
}
