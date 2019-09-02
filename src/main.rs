#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;
extern crate maud;
extern crate base64;

use maud::{html, DOCTYPE, Markup};

#[get("/math")]
fn math() -> Markup {
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

#[post("/base64/enc", data="<input>")]
fn base64_enc(input: Vec<u8>) -> String {
    base64::encode(&input)
}

/*fn base64_dec(input: String) -> Result<Vec<u8>, {
    base64::encode(&input)
}*/

#[post("/base64/dec", data="<input>")]
fn base64_dec(input: String) -> Result<Vec<u8>,&'static str> {
    base64_dec_str(input)
}

#[get("/base64/dec/<input>")]
fn base64_dec_str(input: String) -> Result<Vec<u8>,&'static str> {
    base64::decode(&input).map_err(|_| "Could not decode string")
}

#[get("/base64/enc/<input>")]
fn base64_enc_str(input: String) -> String {
    base64::encode(&input)
}

fn main() {
    rocket::ignite().mount(
        "/",
        routes![
            math,
            base64_dec_str,
            base64_enc_str,
            base64_enc,
            base64_dec
        ]
    ).launch();
}
