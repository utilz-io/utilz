#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;

#[get("/math")]
fn index() -> &'static str {
    "2+2=4"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
