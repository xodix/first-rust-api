use std::usize;

use diesel::{
    associations::HasTable, dsl::count_star, result::QueryResult, GroupByDsl, Insertable,
    MysqlConnection, QueryDsl, Queryable, RunQueryDsl,
};

mod schema {
    table! {
        asortyment (id_asortymentu) {
            id_asortymentu -> Varchar,
            nazwa_asortymentu -> Varchar,
            cena_jednostkowa -> Float,
        }
    }

    table! {
        dane_osobowe (id_klienta) {
            id_klienta -> Varchar,
            imie -> Varchar,
            nazwisko -> Varchar,
            prefix_nip -> Integer,
            nip -> Varchar,
            wojewodztwo -> Varchar,
            kod -> Varchar,
            miejscowosc -> Varchar,
            ulica -> Varchar,
            nr_domu -> Integer,
        }
    }

    table! {
        transakcje (id_transakcji) {
            id_transakcji -> Varchar,
            id_klienta -> Varchar,
            id_asortymentu -> Varchar,
            ilosc -> Float,
            data_transakcji -> Date,
        }
    }

    table! {
        us (id_us) {
            id_us -> Integer,
            nazwa_us -> Varchar,
        }
    }
}

use self::schema::{asortyment, dane_osobowe, transakcje, us};
use self::schema::{
    asortyment::dsl::asortyment as all_asortyment,
    dane_osobowe::dsl::dane_osobowe as all_dane_osobowe,
    transakcje::dsl::transakcje as all_transakcje, us::dsl::us as all_us,
};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "asortyment"]
pub struct Asortyment {
    pub id_asortymentu: String,
    pub nazwa_asortymentu: String,
    pub cena_jednostkowa: f32,
}

impl Asortyment {
    pub fn new(id: String, asortiment_name: String, unit_price: f32) -> Asortyment {
        Asortyment {
            id_asortymentu: id,
            nazwa_asortymentu: asortiment_name,
            cena_jednostkowa: unit_price,
        }
    }
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Asortyment>> {
        asortyment::table.load::<Asortyment>(conn)
    }
    pub fn get_by_id(self, id: String, conn: &MysqlConnection) -> QueryResult<Vec<Asortyment>> {
        all_asortyment.find(id).load::<Asortyment>(conn)
    }
    pub fn get_sorted_by_price(conn: &MysqlConnection) -> QueryResult<Vec<Asortyment>> {
        all_asortyment
            .order(asortyment::cena_jednostkowa)
            .load::<Asortyment>(conn)
    }
    pub fn insert_asortyment(item: Vec<Asortyment>, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(asortyment::table)
            .values(item)
            .execute(conn)
    }
    pub fn len(conn: &MysqlConnection) -> QueryResult<Vec<i64>> {
        all_asortyment.select(count_star()).load(conn)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "dane_osobowe"]
pub struct DaneOsobowe {
    pub id_klienta: String,
    pub imie: String,
    pub nazwisko: String,
    pub prefix_nip: i32,
    pub nip: String,
    pub wojewodztwo: String,
    pub kod: String,
    pub miejscowosc: String,
    pub ulica: String,
    pub nr_domu: i32,
}

impl DaneOsobowe {
    pub fn new(
        id: String,
        name: String,
        surname: String,
        prefix: i32,
        nip_number: String,
        state: String,
        code: String,
        st_name: String,
        a_num: i32,
        town: String,
    ) -> DaneOsobowe {
        DaneOsobowe {
            id_klienta: id,
            imie: name,
            nazwisko: surname,
            prefix_nip: prefix,
            nip: nip_number,
            wojewodztwo: state,
            miejscowosc: town,
            kod: code,
            ulica: st_name,
            nr_domu: a_num,
        }
    }
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<DaneOsobowe>> {
        all_dane_osobowe::table().load(conn)
    }
    pub fn get_by_id(id: String, conn: &MysqlConnection) -> QueryResult<Vec<DaneOsobowe>> {
        all_dane_osobowe.find(id).limit(1).load::<DaneOsobowe>(conn)
    }
    pub fn insert_client(client: Vec<DaneOsobowe>, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(dane_osobowe::table)
            .values(client)
            .execute(conn)
    }
    pub fn len(conn: &MysqlConnection) -> QueryResult<Vec<i64>> {
        all_dane_osobowe.select(count_star()).load(conn)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "transakcje"]
pub struct Transakcje {
    pub id_transakcji: String,
    pub id_klienta: String,
    pub id_asortymentu: String,
    pub ilosc: f32,
    pub data_transakcji: NaiveDate,
}

impl Transakcje {
    pub fn new(
        id_t: String,
        id_k: String,
        id_a: String,
        i: f32,
        d_transakcji: NaiveDate,
    ) -> Transakcje {
        Transakcje {
            id_transakcji: id_t,
            id_klienta: id_k,
            id_asortymentu: id_a,
            ilosc: i,
            data_transakcji: d_transakcji,
        }
    }
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Transakcje>> {
        all_transakcje::table().load::<Transakcje>(conn)
    }
    pub fn get_by_id(id: String, conn: &MysqlConnection) -> QueryResult<Vec<Transakcje>> {
        all_transakcje::table().find(id).load::<Transakcje>(conn)
    }
    pub fn sum_ilosc(conn: &MysqlConnection) -> QueryResult<Vec<Transakcje>> {
        all_transakcje::table()
            .select(transakcje::id_asortymentu)
            .group_by(transakcje::id_asortymentu)
            .select(diesel::dsl::sum(transakcje::ilosc))
            .select(transakcje::all_columns)
            .load(conn)
    }
    pub fn insert_transakcje(vals: Vec<Transakcje>, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(transakcje::table)
            .values(vals)
            .execute(conn)
    }
    pub fn len(conn: &MysqlConnection) -> QueryResult<Vec<i64>> {
        all_transakcje.select(count_star()).load(conn)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "us"]
pub struct Us {
    pub id_us: i32,
    pub nazwa_us: String,
}

impl Us {
    pub fn new(id: i32, nazwa: String) -> Us {
        Us {
            id_us: id,
            nazwa_us: nazwa,
        }
    }
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Us>> {
        all_us::table().load(conn)
    }
    pub fn get_by_id(conn: &MysqlConnection, id: i32) -> QueryResult<Vec<Us>> {
        all_us::table().find(id).load::<Us>(conn)
    }
    pub fn insert_new_us(conn: &MysqlConnection, vals: Vec<Us>) -> QueryResult<usize> {
        diesel::insert_into(us::table).values(vals).execute(conn)
    }
    pub fn len(conn: &MysqlConnection) -> QueryResult<Vec<i64>> {
        all_us.select(count_star()).load(conn)
    }
}
