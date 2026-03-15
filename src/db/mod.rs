use anyhow::anyhow;
use rusqlite::Connection;

pub struct Db {
    connection: Option<Connection>,
}

pub trait ConnectionTrait {
    fn connection(&self) -> Option<&Connection>;
}

impl Db {
    pub fn new() -> Self {
        Self { connection: None }
    }

    pub fn connect_memory(&mut self) -> anyhow::Result<()> {
        if self.connection.is_some() {
            return Err(anyhow!("Already connected"));
        }

        let Ok(conn) = Connection::open_in_memory() else {
            return Err(anyhow!("Failed to connected"));
        };

        self.connection = Some(conn);

        Ok(())
    }

    pub fn connect_file(&mut self, path: &str) -> anyhow::Result<()> {
        if self.connection.is_some() {
            return Err(anyhow!("Already connected"));
        }

        let Ok(conn) = Connection::open(path) else {
            return Err(anyhow!("Failed to connected"));
        };

        self.connection = Some(conn);

        Ok(())
    }

    pub fn init_tables(&self) -> anyhow::Result<()> {
        let Some(conn) = self.connection.as_ref() else {
            return Err(anyhow!("Database not connected"));
        };

        conn.execute(
            "
        CREATE table if not exists nodes (
            id integer primary key autoincrement,
            title text not null,
            url text,
            created_at datetime default current_timestamp,
            updated_at datetime default current_timestamp
        )",
            (),
        )?;

        Ok(())
    }

    pub fn seed_tables(&self) -> anyhow::Result<()> {
        let Some(conn) = self.connection.as_ref() else {
            return Err(anyhow!("Database not connected"));
        };

        let node1 = ("Node 1", None::<&str>);
        let node2 = ("My Blog", Some("https://bobman.dev/blog"));
        let node3 = ("Zed Blog", Some("https://zed.dev/blog"));

        for (title, url) in [node1, node2, node3] {
            conn.execute(
                "insert into nodes (title, url) values (?1, ?2)",
                (title, url),
            )?;
        }

        Ok(())
    }
}

impl ConnectionTrait for Db {
    fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }
}
