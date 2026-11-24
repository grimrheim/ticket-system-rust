use rusqlite::{Connection, Result, params};

const DB_PATH: &str = "./tickets.db";

#[derive(Debug)]
pub struct SqliteStorage {
    pub(crate) conn: Connection
}

impl SqliteStorage {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let schema = "CREATE TABLE IF NOT EXISTS tickets (
                            id          INTEGER PRIMARY KEY,
                            header      TEXT NOT NULL,
                            description TEXT,
                            created     TEXT NOT NULL,
                            expires     TEXT NOT NULL,
                            path        TEXT,
                            status      TEXT NOT NULL,
                            priority    TEXT NOT NULL,
                            parent      INTEGER
                            )";

        let conn = match path {
            Some(path)  => Connection::open(path)?,
            None        => Connection::open_in_memory()?
        };

        conn.execute(schema,())?;

        Ok(Self { conn })
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // Result - тип для операций, которые могут не удастся.   Может вернуть Ok или Err
    // ? - короткая запись для логики "если Ok, достань значение и иди дальше;
    // если Err, немедленно выйди и верни эту ошибку"
    fn connection_sql() -> rusqlite::Result<()> {
        let conn = SqliteStorage::new(Some("test.db"))?;
        assert!(std::path::Path::new("test.db").exists());
        let _ = std::fs::remove_file("test.db");

        // let ticket = make_ticket();
        //
        // conn.execute(
        //     "INSERT INTO tickets (id, header, description)
        //             VALUES (?1, ?2, ?3)",
        // params![ticket.id.0, ticket.header, ticket.description],
        // )?;
        //
        // let res = conn.query_row(
        //     "SELECT header, description FROM tickets WHERE id = ?1",
        //     params![ticket.id.0],
        //     // Аналогично нужен ? для корректной логики обработки ошибки
        //     |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        // )?;
        //
        // assert_eq!(res.0, ticket.header);
        // assert_eq!(res.1, ticket.description);
        //
        // Ok(())
        Ok(())
    }

}
