use diesel::{RunQueryDsl, SqliteConnection};

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn last_inserted_id(conn: &SqliteConnection) -> i32 {
    return diesel::select(last_insert_rowid)
    .first(conn)
    .expect("Nothing lastly inserted");
}