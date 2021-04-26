#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket_contrib::json::{self, Json};
use rocket_contrib::templates::Template;
use serde_derive::{Deserialize, Serialize};
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
struct JData<T> {
    all: T,
}

#[database("msql")]
pub struct LogsDbConn(diesel::mysql::MysqlConnection);

#[get("/")]
fn index(conn: LogsDbConn) -> Template {
    let all_data = models::DaneOsobowe::get_all(&conn).unwrap();
    Template::render("index", json!({ "all": all_data }))
}
#[derive(Debug, Deserialize, Serialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[get("/sss", data = "<color>")]
fn sss(color: Json<Color>) -> json::JsonValue {
    json!({"a": color.r, "b": color.b})
}

#[get("/ping")]
fn ping() -> rocket::response::content::Html<&'static str> {
    rocket::response::content::Html("<b>pong</b>")
}

#[get("/asortyment")]
fn asortyment(conn: LogsDbConn) -> json::JsonValue {
    let asortyment_all = models::Asortyment::get_all(&conn).unwrap();
    json!(asortyment_all)
}

#[get("/dane_osobowe")]
fn dane_osobowe(conn: LogsDbConn) -> json::JsonValue {
    let dane_all = models::DaneOsobowe::get_all(&conn).unwrap();
    json!(dane_all)
}

#[get("/dane_osobowe/length")]
fn dane_length(conn: LogsDbConn) -> String {
    let res = models::DaneOsobowe::len(&conn).unwrap()[0];
    format!("{}", res)
}

#[get("/transakcje")]
fn transakcje(conn: LogsDbConn) -> json::JsonValue {
    let transakcje_all = models::Transakcje::get_all(&conn).unwrap();
    json!(transakcje_all)
}

#[get("/us")]
fn us(conn: LogsDbConn) -> json::JsonValue {
    let us_all = models::Us::get_all(&conn).unwrap();
    json!(us_all)
}

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                asortyment,
                dane_osobowe,
                transakcje,
                us,
                dane_length,
                ping,
                sss
            ],
        )
        .launch();
}
