#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;
extern crate maud;
extern crate base64;

use rocket::response::content::Plain;
use rocket::response::Responder;
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
fn base64_enc(input: Vec<u8>) -> impl Responder<'static> {
    base64::encode(&input)
}

#[post("/base64/dec", data="<input>")]
fn base64_dec(input: String) -> impl Responder<'static> {
    base64_dec_str(input)
}

#[get("/base64/dec/<input>")]
fn base64_dec_str(input: String) -> impl Responder<'static> {
    base64::decode(&input).map_err(|_| "Could not decode string")
}

#[get("/base64/enc/<input>")]
fn base64_enc_str(input: String) -> impl Responder<'static> {
    base64::encode(&input)
}

#[get("/whois/<domain>")]
fn whois(domain: String) -> impl Responder<'static> {
    use std::process::Command;
    let mut out = Command::new("whois")
        .arg(&domain)
        .output()
        .expect("failed to execute process");
    let mut res:Vec<u8> = Vec::new();
    res.append(&mut out.stdout);
    res.append(&mut out.stderr);
    Plain(res)
}

// #[get("/delay/<ms>")]
// fn delay(ms: u64) -> String

fn main() {
    let cors = rocket_cors::CorsOptions{
        allow_credentials: true,
        .. Default::default()
    }.to_cors().unwrap();
    rocket::ignite().mount(
        "/",
        routes![
            math,
            base64_dec_str,
            base64_enc_str,
            base64_enc,
            base64_dec,
            whois
        ]
    ).attach(cors)
    .launch();
}
