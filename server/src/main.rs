#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::fairing::{Fairing, Info, Kind};
use sql;
use account;
use account_assets;

#[derive(Serialize)]
struct TemplateContext {
    context : String
}


#[get("/")]
fn index() -> Template {
    let context = account_assets::account_assets();
    let context = TemplateContext{context};
    Template::render("index", &context)

}
   /* "Hello, I am server".to_string()*/
    //account_assets::account_assets()



#[get("/accounting")]
fn accounting() -> String {
    account::account()
}


fn main() {
    rocket::ignite()
    .mount("/", routes![index, accounting])
    .attach(Template::fairing())
    .launch();
}





/*
fn run() {
   println!("Hello, world, I am the server");

    sql::sql();
    account::account();
    account_assets::account_assets();
}
*/