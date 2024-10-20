use rocket::post;

#[post("/upload")]
pub fn upload() -> &'static str {
    "hi"
}
