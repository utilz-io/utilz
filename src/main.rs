#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;
extern crate maud;

use maud::{html, DOCTYPE, Markup};

#[get("/math")]
fn index() -> Markup {
    let inner_str = "2+2=4";
    html! {
        (DOCTYPE)
        head {
            title { "Some awesome math" }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no";
        }
        body {
            p {
                "Hey look at this awesome math" .math { (inner_str) }
            }
        }
    }       
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
