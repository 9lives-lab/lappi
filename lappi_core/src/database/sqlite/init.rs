use rusqlite::Connection;

pub fn get_tables_list() -> Vec<&'static str> {
    return vec![
        "folders",
        "internal_files",
        "music_items",
        "lyrics_items",
        "picture_items",
        "tags",
        "music_files",
        "music_sources",
        "playlists",
        "playlist_items"
    ];
}

pub fn create_tables(connection: &Connection) -> rusqlite::Result<usize> {

    connection.execute(
        "CREATE TABLE folders (
                id                              INTEGER NOT NULL PRIMARY KEY,
                parent_id                       INTEGER,
                name                            TEXT    NOT NULL,
                folder_type                     INTEGER NOT NULL,
                avatar_picture_id               INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE internal_files (
                id                              INTEGER NOT NULL PRIMARY KEY,
                internal_path                   TEXT    NOT NULL
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_items (
                id                              INTEGER NOT NULL PRIMARY KEY,
                name                            TEXT    NOT NULL,
                folder_id                       INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE lyrics_items (
                id                              INTEGER NOT NULL PRIMARY KEY,
                music_item_id                   INTEGER NOT NULL,
                lang_code                       TEXT    NOT NULL,
                FOREIGN KEY(music_item_id)      REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE picture_items (
                id                              INTEGER NOT NULL PRIMARY KEY,
                extension                       TEXT    NOT NULL,
                folder_id                       INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE tags (
                id                              INTEGER NOT NULL PRIMARY KEY,
                music_item_id                   INTEGER,
                folder_id                       INTEGER,
                tag_name                        TEXT    NOT NULL,
                string_value                    TEXT,
                int_value                       INTEGER,
                FOREIGN KEY(music_item_id)      REFERENCES music_items(id),
                FOREIGN KEY(folder_id)          REFERENCES folders(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_files (
                id                              INTEGER NOT NULL UNIQUE,
                internal_file_id                INTEGER NOT NULL,
                file_type                       INTEGER NOT NULL,
                FOREIGN KEY(id)                 REFERENCES music_items(id),
                FOREIGN KEY(internal_file_id)   REFERENCES internal_files(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE music_links (
                id                              INTEGER NOT NULL PRIMARY KEY,
                music_item_id                   INTEGER NOT NULL,
                link                            TEXT,
                link_type                       INTEGER NOT NULL,
                FOREIGN KEY(music_item_id)      REFERENCES music_items(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlists (
                id                              INTEGER NOT NULL PRIMARY KEY,
                name                            TEXT    NOT NULL UNIQUE,
                avatar_picture_id               INTEGER
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE playlist_items (
                id                              INTEGER NOT NULL PRIMARY KEY,
                playlist_id                     INTEGER NOT NULL,
                music_item_id                   INTEGER,
                FOREIGN KEY(playlist_id)        REFERENCES playlists(id)
        )",
        [],
    )?;

    return Ok(0);
}

