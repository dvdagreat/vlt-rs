use rusqlite::{Connection, Result, params};
use std::fs;

pub struct Db {
    conn: Connection,
    path: String,
}

pub struct CredentialRow {
    pub secret: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl Db {
    pub fn init(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS credentials (
                id INTEGER PRIMARY KEY,
                service TEXT NOT NULL,
                identifier TEXT NOT NULL,
                secret BLOB NOT NULL,
                nonce BLOB NOT NULL
            )",
            [],
        )?;
        Ok(Self {
            conn,
            path: path.to_string(),
        })
    }

    pub fn add_credential(
        &self,
        service: &str,
        ident: &str,
        secret: &[u8],
        nonce: &[u8],
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO credentials (service, identifier, secret, nonce) VALUES (?1, ?2, ?3, ?4)",
            params![service, ident, secret, nonce],
        )?;
        Ok(())
    }

    pub fn get_credential(&self, service: &str) -> Result<CredentialRow> {
        self.conn.query_row(
            "SELECT secret, nonce FROM credentials WHERE service = ?1",
            params![service],
            |row| {
                Ok(CredentialRow {
                    secret: row.get(0)?,
                    nonce: row.get(1)?,
                })
            },
        )
    }

    pub fn delete_all_data(&self) -> std::io::Result<()> {
        // The Nuclear Option
        fs::remove_file(&self.path)
    }
}
