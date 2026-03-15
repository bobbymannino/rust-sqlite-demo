use chrono::{DateTime, Utc};
use rusqlite::Connection;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let conn = create_connection()?;
    init_tables(&conn)?;
    seed_tables(&conn)?;
    let nodes = Node::get_all(&conn)?;
    for node in nodes {
        node.pretty_print();
    }

    Ok(())
}

struct Node {
    id: u32,
    title: String,
    url: Option<String>,
    createdAt: DateTime<Utc>,
    updatedAt: DateTime<Utc>,
}

impl Node {
    fn pretty_print(&self) {
        println!("Node #{}", self.id);
        println!("  Title: {}", self.title);
        println!("  URL: {:?}", self.url);
        println!("  Created At: {}", self.createdAt);
        println!("  Updated At: {}", self.updatedAt);
    }

    fn get_all(conn: &Connection) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn
            .prepare("select id, title, url, created_at, updated_at from nodes order by id desc")?;
        let nodes = stmt.query_map([], |r| {
            Ok(Node {
                id: r.get(0)?,
                title: r.get(1)?,
                url: r.get(2)?,
                createdAt: r.get(3)?,
                updatedAt: r.get(4)?,
            })
        })?;
        let nodes_vec: Vec<Node> = nodes.collect::<Result<Vec<_>, _>>()?;
        Ok(nodes_vec)
    }
}

fn create_connection() -> anyhow::Result<Connection> {
    Ok(Connection::open_in_memory()?)
}

fn init_tables(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "
        CREATE table nodes (
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

fn seed_tables(conn: &Connection) -> anyhow::Result<()> {
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
