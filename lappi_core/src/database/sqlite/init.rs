use rusqlite::{Connection};

pub fn get_tables_list() -> Vec<&'static str> {
    return vec![
        "collection_items",
        "collection_folders",
        "artist_entries",
        "artist_connections",
        "tags_names",
        "tags_values",
        "tags",
        "external_src_files",
    ];
}

pub fn create_tables(connection: &Connection) -> rusqlite::Result<usize> {

    connection.execute(
        "CREATE TABLE collection_items (
                id             INTEGER NOT NULL PRIMARY KEY,
                creation_date  INTEGER NOT NULL,
                folder_id      INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE artist_entries (
                id             INTEGER NOT NULL PRIMARY KEY,
                name           TEXT    NOT NULL UNIQUE,
                folder_id      INTEGER,
                FOREIGN KEY(folder_id)  REFERENCES collection_folders(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE artist_connections (
                id             INTEGER NOT NULL PRIMARY KEY,
                item_id        INTEGER NOT NULL,
                artist_id      INTEGER NOT NULL,
                FOREIGN KEY(item_id)    REFERENCES collection_items(id),
                FOREIGN KEY(artist_id)  REFERENCES artist_entries(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags_names (
                id    INTEGER NOT NULL PRIMARY KEY,
                name  TEXT    NOT NULL UNIQUE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags_values (
                id       INTEGER NOT NULL PRIMARY KEY,
                name_id  INTEGER NOT NULL,
                value    TEXT NOT NULL,
                FOREIGN KEY(name_id)  REFERENCES tags_names(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags (
                id       INTEGER NOT NULL PRIMARY KEY,
                item_id  INTEGER NOT NULL,
                tag_id   INTEGER NOT NULL,
                FOREIGN KEY(item_id) REFERENCES collection_items(id),
                FOREIGN KEY(tag_id)  REFERENCES tags_values(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE external_src_files (
                id       INTEGER NOT NULL PRIMARY KEY,
                item_id  INTEGER NOT NULL,
                path     TEXT    NOT NULL,
                FOREIGN KEY(item_id) REFERENCES collection_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE collection_folders (
                id          INTEGER NOT NULL PRIMARY KEY,
                parent_id   INTEGER,
                name        TEXT NOT NULL,
                folder_type INTEGER NOT NULL
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE classic_playlists (
                id    INTEGER NOT NULL PRIMARY KEY,
                name  TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE classic_playlist_items (
                id           INTEGER NOT NULL PRIMARY KEY,
                playlist_id  INTEGER NOT NULL,
                music_item_id  INTEGER,
                FOREIGN KEY(playlist_id) REFERENCES classic_playlists(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE collection_nodes (
                id             INTEGER NOT NULL PRIMARY KEY,
                type           INTEGER NOT NULL,
                name           TEXT    NOT NULL,
                creation_date  INTEGER NOT NULL,
                folder_id      INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE collection_edges (
                id             INTEGER NOT NULL PRIMARY KEY,
                first_node_id  INTEGER NOT NULL,
                second_node_id INTEGER NOT NULL,
                FOREIGN KEY(first_node_id)  REFERENCES collection_nodes(id),
                FOREIGN KEY(second_node_id) REFERENCES collection_nodes(id)
        )",
        [],
    )?;

    return Ok(0);
}

