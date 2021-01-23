#[rocket_contrib::database("db")]
pub struct Database(diesel::SqliteConnection);
