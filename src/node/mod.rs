use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub struct Node {
    id: u32,
    title: String,
    url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Node {
    pub fn pretty_print(&self) {
        println!("Node #{}", self.id);
        println!("  Title: {}", self.title);
        println!("  URL: {:?}", self.url);
        println!("  Created At: {}", self.created_at);
        println!("  Updated At: {}", self.updated_at);
    }

    pub fn get_all(conn: &Connection) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn
            .prepare("select id, title, url, created_at, updated_at from nodes order by id desc")?;
        let nodes = stmt.query_map([], |r| {
            Ok(Node {
                id: r.get(0)?,
                title: r.get(1)?,
                url: r.get(2)?,
                created_at: r.get(3)?,
                updated_at: r.get(4)?,
            })
        })?;
        let nodes_vec: Vec<Node> = nodes.collect::<Result<Vec<_>, _>>()?;
        Ok(nodes_vec)
    }
}
