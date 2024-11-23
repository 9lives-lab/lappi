use rusqlite::Connection;

pub fn get_tables_list() -> Vec<&'static str> {
    return vec![
        "folders",
        "music_items",
        "lyrics_items",
        "picture_items",
        "tags_names",
        "tags_values",
        "music_items_tags",
        "music_src_files",
    ];
}

pub fn create_tables(connection: &Connection) -> rusqlite::Result<usize> {

    connection.execute(
        "CREATE TABLE folders (
                id                      INTEGER NOT NULL PRIMARY KEY,
                parent_id               INTEGER,
                name                    TEXT    NOT NULL,
                folder_type             INTEGER NOT NULL,
                avatar_picture_id       INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_items (
                id                      INTEGER NOT NULL PRIMARY KEY,
                name                    TEXT    NOT NULL,
                folder_id               INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE lyrics_items (
                id                      INTEGER NOT NULL PRIMARY KEY,
                music_item_id           INTEGER NOT NULL,
                lang_code               TEXT    NOT NULL,
                FOREIGN KEY(music_item_id) REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE picture_items (
                id                      INTEGER NOT NULL PRIMARY KEY,
                extension               TEXT    NOT NULL,
                folder_id               INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags_names (
                id                      INTEGER NOT NULL PRIMARY KEY,
                name                    TEXT    NOT NULL UNIQUE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags_values (
                id                      INTEGER NOT NULL PRIMARY KEY,
                name_id                 INTEGER NOT NULL,
                value                   TEXT    NOT NULL,
                FOREIGN KEY(name_id)    REFERENCES tags_names(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_items_tags (
                id                      INTEGER NOT NULL PRIMARY KEY,
                item_id                 INTEGER NOT NULL,
                tag_id                  INTEGER NOT NULL,
                FOREIGN KEY(item_id)    REFERENCES music_items(id),
                FOREIGN KEY(tag_id)     REFERENCES tags_values(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_src_files (
                id                      INTEGER NOT NULL PRIMARY KEY,
                music_item_id           INTEGER NOT NULL,
                source_type             INTEGER NOT NULL,
                path                    TEXT    NOT NULL,
                FOREIGN KEY(music_item_id) REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlists (
                id                      INTEGER NOT NULL PRIMARY KEY,
                name                    TEXT    NOT NULL UNIQUE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlist_items (
                id                      INTEGER NOT NULL PRIMARY KEY,
                playlist_id             INTEGER NOT NULL,
                music_item_id           INTEGER,
                FOREIGN KEY(playlist_id) REFERENCES playlists(id)
        )",
        [],
    )?;

    return Ok(0);
}

