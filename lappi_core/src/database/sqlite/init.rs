use rusqlite::Connection;

pub fn get_tables_list() -> Vec<&'static str> {
    return vec![
        "folders",
        "music_items",
        "lyrics_items",
        "picture_items",
        "tags",
        "music_src_files",
        "playlists",
        "playlist_items"
    ];
}

pub fn create_tables(connection: &Connection) -> rusqlite::Result<usize> {

    connection.execute(
        "CREATE TABLE folders (
                id                          INTEGER NOT NULL PRIMARY KEY,
                parent_id                   INTEGER,
                name                        TEXT    NOT NULL,
                folder_type                 INTEGER NOT NULL,
                avatar_picture_id           INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_items (
                id                          INTEGER NOT NULL PRIMARY KEY,
                name                        TEXT    NOT NULL,
                folder_id                   INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE lyrics_items (
                id                          INTEGER NOT NULL PRIMARY KEY,
                music_item_id               INTEGER NOT NULL,
                lang_code                   TEXT    NOT NULL,
                FOREIGN KEY(music_item_id)  REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE picture_items (
                id                          INTEGER NOT NULL PRIMARY KEY,
                extension                   TEXT    NOT NULL,
                folder_id                   INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags (
                id                          INTEGER NOT NULL PRIMARY KEY,
                music_item_id               INTEGER,
                folder_id                   INTEGER,
                tag_name                    TEXT    NOT NULL,
                string_value                TEXT,
                int_value                   INTEGER,
                FOREIGN KEY(music_item_id)  REFERENCES music_items(id),
                FOREIGN KEY(folder_id)      REFERENCES folders(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_src_files (
                id                          INTEGER NOT NULL PRIMARY KEY,
                music_item_id               INTEGER NOT NULL,
                source_type                 INTEGER NOT NULL,
                path                        TEXT    NOT NULL,
                FOREIGN KEY(music_item_id)  REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlists (
                id                          INTEGER NOT NULL PRIMARY KEY,
                name                        TEXT    NOT NULL UNIQUE,
                avatar_picture_id           INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlist_items (
                id                          INTEGER NOT NULL PRIMARY KEY,
                playlist_id                 INTEGER NOT NULL,
                music_item_id               INTEGER,
                FOREIGN KEY(playlist_id)    REFERENCES playlists(id)
        )",
        [],
    )?;

    return Ok(0);
}

