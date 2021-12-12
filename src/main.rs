use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "create table if not exists notes (
      id integer primary key,
      body text not null unique
    )",
        [],
    )?;
    Ok(())
}
