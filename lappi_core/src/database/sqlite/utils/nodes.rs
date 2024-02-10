use num_traits::FromPrimitive;
use rusqlite::{params, Connection, OptionalExtension};

use crate::collection::types::{EdgeId, ItemId, ItemType};

pub fn add_node(conn: &Connection, item_type: ItemType, name: &str) -> ItemId {
    conn.execute(
        "INSERT INTO collection_nodes (type, name, creation_date) VALUES (?1, ?2, datetime('now'))",
        params![item_type as i32, name],
    ).unwrap();
    return conn.last_insert_rowid();
}

pub fn find_node_id(conn: &Connection, item_type: ItemType, name: &str) -> Option<ItemId> {
    return conn.query_row(
        "SELECT id FROM collection_nodes WHERE type == (?1) AND name == (?2)",
        params![item_type as i32, name],
        |row| row.get::<_, i64>(0),
    ).optional().unwrap();
}

pub fn get_node_type(conn: &Connection, item_id: ItemId) -> ItemType {
    let type_number = conn.query_row(
        "SELECT type FROM collection_nodes WHERE id=(?1)",
        params![item_id],
        |row| row.get::<_, i64>(0),
    ).unwrap();
    return ItemType::from_i32(type_number as i32).unwrap();
}

pub fn get_node_name(conn: &Connection, item_id: ItemId) -> String {
    return conn.query_row(
        "SELECT name FROM collection_nodes WHERE id=(?1)",
        params![item_id],
        |row| row.get::<_, String>(0),
    ).unwrap();
}

pub fn add_edge(conn: &Connection, first_node: ItemId, second_node: ItemId) -> ItemId {
    conn.execute(
        "INSERT INTO collection_edges (first_node_id, second_node_id) VALUES (?1, ?2)",
        params![first_node, second_node],
    ).unwrap();
    return conn.last_insert_rowid();
}

pub fn find_edge_id(conn: &Connection, first_node: ItemId, second_node: ItemId) -> Option<ItemId> {
    return conn.query_row(
        "SELECT id FROM collection_edges WHERE first_node_id == (?1) AND second_node_id == (?2)",
        params![first_node, second_node],
        |row| row.get::<_, i64>(0),
    ).optional().unwrap();
}

pub fn get_edge(conn: &Connection, edge_id: EdgeId) -> (ItemId, ItemId) {
    return conn.query_row(
        "SELECT first_node_id, second_node_id FROM collection_edges WHERE id=(?1)",
        params![edge_id],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
    ).unwrap();
}

pub fn get_all_edges(conn: &Connection) -> Vec<EdgeId> {
    let mut collection_stmt = conn.prepare("SELECT id FROM collection_edges").unwrap();
    let edge_rows = collection_stmt.query_map(
        [],|row| row.get::<_, i64>(0)
    ).unwrap();
    let mut result = Vec::new();
    for edge_id in edge_rows {
        result.push(edge_id.unwrap());
    }
    return result;
}
