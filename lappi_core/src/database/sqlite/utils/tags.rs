use rusqlite::{params, Connection, OptionalExtension};
use crate::database_api::DbResult;

pub fn add_tag_to_item(conn: &Connection, collection_item_id: i64, tag_value_id: i64) -> DbResult<()> {
    conn.execute(
        "INSERT INTO music_items_tags (item_id, tag_id) VALUES (?1, ?2)",
        params![collection_item_id, tag_value_id],
    )?;
    Ok(())
}

pub fn get_add_tag_value(conn: &Connection, name_id: i64, value: &str) -> DbResult<i64> {
    let result = conn.query_row(
        "SELECT id FROM tags_values WHERE name_id=(?1) AND value=(?2)",
        params![name_id, value],
        |row| row.get::<_, i64>(0),
    ).optional()?;
    let value_id = match result {
        Some(id) => id,
        None => {
            conn.execute(
                "INSERT INTO tags_values (name_id, value) VALUES (?1, ?2)",
                params![name_id, value],
            )?;
            conn.last_insert_rowid()
        }
    };
    Ok(value_id)
}

pub fn get_tag_name_id(conn: &Connection, name: &str) -> DbResult<Option<i64>> {
    let result = conn.query_row(
        "SELECT id FROM tags_names WHERE name=(?1)",
        params![name],
        |row| row.get::<_, i64>(0),
    ).optional()?;
    Ok(result)
}

pub fn get_add_tag_name(conn: &Connection, name: &str) -> DbResult<i64> {
    let name_id = match get_tag_name_id(conn, name)? {
        Some(id) => id,
        None => {
            conn.execute(
                "INSERT INTO tags_names (name) VALUES (?1)",
                params![name],
            )?;
            conn.last_insert_rowid()
        }
    };
    Ok(name_id)
}
