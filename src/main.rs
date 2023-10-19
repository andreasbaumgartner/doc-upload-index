#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Document Uploader"
}

#[get("/upload/")]
fn upload() -> &'static str {
    "Upload a file"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
