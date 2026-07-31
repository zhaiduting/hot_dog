use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB:rusqlite::Connection = {
        let conn=rusqlite::Connection::open("dogs.db").expect("Failed to open dog database");
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL
            )
        ").unwrap();
        conn
    }
}

#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<()> {
    use rusqlite::params;
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?);", params![image]))?;
    Ok(())
}
