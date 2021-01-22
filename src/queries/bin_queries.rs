use diesel::{insert_into, RunQueryDsl};
use diesel::{prelude::*, result::Error};

use crate::bins::dsl::*;
use crate::models::{Bin, NewBin};
use crate::schema::bins;

use super::helpers::last_inserted_id;

pub fn create_bin<'a>(conn: &SqliteConnection, new_bin: &NewBin) -> Result<Bin, Error> {
    insert_into(bins::table).values(new_bin).execute(conn)?;

    Ok(get_bin(conn, last_inserted_id(conn))?)
}

pub fn get_bins(conn: &SqliteConnection) -> Result<Vec<Bin>, Error> {
    Ok(bins.get_results(conn)?)
}

pub fn get_bin(conn: &SqliteConnection, bin_id: i32) -> Result<Bin, Error> {
    Ok(bins.find(bin_id).first(conn)?)
}

pub fn update_bin(
    conn: &SqliteConnection,
    bin_id: i32,
    bin_title: &str,
    bin_code: &str,
) -> Result<Bin, Error> {
    diesel::update(bins.find(bin_id))
        .set((title.eq(bin_title), code.eq(bin_code)))
        .execute(conn)?;

    Ok(get_bin(conn, last_inserted_id(conn))?)
}

pub fn delete_bin(conn: &SqliteConnection, bin_id: i32) -> Result<usize, Error> {
    Ok(diesel::delete(bins.find(bin_id)).execute(conn)?)
}
