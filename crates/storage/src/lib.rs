use rusqlite::{Connection, Result, params};
use std::fs;

pub struct Db {
    conn: Connection,
    path: String,
}

pub struct CredentialRow {
    pub id: i32,
    pub secret: Vec<u8>,
    pub nonce: Vec<u8>,
    pub identifier: String,
}

pub struct MasterPasswordRow {
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

    pub fn edit_credential(
        &self,
        service: &str,
        ident: &str,
        secret: &[u8],
        nonce: &[u8],
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE credentials SET secret = ?3, nonce = ?4 WHERE service = ?1 AND identifier = ?2",
            params![service, ident, secret, nonce],
        )?;
        Ok(())
    }

    pub fn update_credential(
        &self,
        service: &str,
        ident: &str,
        secret: &[u8],
        nonce: &[u8],
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE credentials SET secret = ?3, nonce = ?4 WHERE service = ?1 AND identifier = ?2",
            params![service, ident, secret, nonce],
        )?;
        Ok(())
    }

    pub fn update_credential_by_id(&self, id: i32, secret: &[u8], nonce: &[u8]) -> Result<()> {
        self.conn.execute(
            "UPDATE credentials SET secret = ?2, nonce = ?3 WHERE id = ?1",
            params![id, secret, nonce],
        )?;
        Ok(())
    }

    pub fn get_random_credential(&self) -> Result<CredentialRow> {
        self.conn.query_row(
            "SELECT id, secret, nonce, identifier FROM credentials ORDER BY RANDOM() LIMIT 1",
            params![],
            |row| {
                Ok(CredentialRow {
                    id: row.get(0)?,
                    secret: row.get(1)?,
                    nonce: row.get(2)?,
                    identifier: row.get(3)?,
                })
            },
        )
    }

    pub fn get_batch_credentials(&self, offset: i64, limit: i64) -> Result<Vec<CredentialRow>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, secret, nonce, identifier FROM credentials LIMIT ?1 OFFSET ?2")?;
        let credential_iter = stmt.query_map(params![limit, offset], |row| {
            Ok(CredentialRow {
                id: row.get(0)?,
                secret: row.get(1)?,
                nonce: row.get(2)?,
                identifier: row.get(3)?,
            })
        })?;

        let mut credentials = Vec::new();
        for credential in credential_iter {
            credentials.push(credential?);
        }
        Ok(credentials)
    }

    pub fn get_credential(&self, service: &str, identifier: &str) -> Result<CredentialRow> {
        self.conn.query_row(
            "SELECT id, secret, nonce, identifier FROM credentials WHERE service = ?1 AND identifier = ?2",
            params![service, identifier],
            |row| {
                Ok(CredentialRow {
                    id: row.get(0)?,
                    secret: row.get(1)?,
                    nonce: row.get(2)?,
                    identifier: row.get(3)?,
                })
            },
        )
    }

    pub fn delete_credential(&self, service: &str, identifier: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM credentials WHERE service = ?1 AND identifier = ?2",
            params![service, identifier],
        )?;
        Ok(())
    }

    pub fn delete_all_data(&self) -> std::io::Result<()> {
        // The Nuclear Option
        fs::remove_file(&self.path)
    }

    pub fn get_master_password(&self) -> Result<Option<MasterPasswordRow>, String> {
        let result = self.conn.query_row(
            "select secret, nonce from master_passwords;",
            params![],
            |row| {
                Ok(MasterPasswordRow {
                    secret: row.get(0)?,
                    nonce: row.get(1)?,
                })
            },
        );

        match result {
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(_) => Err("Internal Server Error".to_owned()),
            Ok(result) => Ok(Some(result)),
        }
    }

    pub fn create_service(&self) {}
}
