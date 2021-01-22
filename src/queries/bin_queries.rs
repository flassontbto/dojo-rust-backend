use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};

use crate::bins::dsl::*;
use crate::models::{Bin, NewBin};
use crate::schema::bins;

use super::helpers::last_inserted_id;

pub fn create_bin<'a>(conn: &SqliteConnection, bin_title: &'a str, bin_author: i32, bin_code: &'a str) -> Bin {
    let new_bin = NewBin {
        title: bin_title,
        author_id: bin_author,
        code: bin_code,
    };

    insert_into(bins::table)
        .values(&new_bin)
        .execute(conn)
        .expect("Error saving new bin");

    return get_bin(conn, last_inserted_id(conn));
}

pub fn get_bins(conn: &SqliteConnection) -> Vec<Bin> {
    return bins.get_results(conn).expect("Error loading bins");
}

pub fn get_bin(conn: &SqliteConnection, bin_id: i32) -> Bin {
    return bins
        .find(bin_id)
        .first(conn)
        .expect("Unable to retrieve bin");
}

pub fn update_bin(conn: &SqliteConnection, bin_id: i32, bin_title: &str, bin_code: &str) -> Bin {
    diesel::update(bins.find(bin_id))
        .set((title.eq(bin_title), code.eq(bin_code)))
        .execute(conn)
        .expect("Error updating bin");

    return get_bin(conn, last_inserted_id(conn));
}

pub fn delete_bin(conn: &SqliteConnection, bin_id: i32) -> usize {
    return diesel::delete(bins.find(bin_id))
        .execute(conn)
        .expect("Error deleting user");
}
