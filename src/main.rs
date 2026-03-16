mod db;
mod node;

use crate::{db::Db, node::Node};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut db = Db::new();
    db.connect_file("test.db")?;
    db.migrate()?;
    db.seed_tables()?;
    let nodes = Node::get_all(db)?;
    for node in nodes {
        node.pretty_print();
    }

    Ok(())
}
