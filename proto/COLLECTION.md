# Collection Format

Lappi collection is a folder containing music files and set of metadata files.
Metadata files are stored in the `.lappi` folder in the root of the collection.

Collection folder structure:
- `collection_root/`
    - `.lappi/`
        - `meta/`
           - `folders.pb`
           - `lyrics.pb`
           - `music_items.pb`
           - `music_src_files.pb`
           - `picture_items.pb`
           - `playlist_items.pb`
           - `playlists.pb`
           - `tags.pb`
    - `Artist 1/`
        - `2000 - Album 1/`
           - `01 - Track 1.mp3`
           - `02 - Track 2.mp3`
           - `03 - Track 3.mp3`
    - `Artist 2/`
    - ...

## Metadata Format

Metadata is serialized using [Protocol Buffers](https://protobuf.dev).

Base element of metadata structure is table. Each table is stored in a separate file.
Tables are stored in the `.lappi/meta` folder.

The table rows are written to the file sequentially row by row.
Each row is prefixed with a 4-byte length of the row in little-endian format.
Each row is serialized as a Protocol Buffers message.

## Tables

### `folders.pb`

Describes folders in the collection.
Rows are encoded as `FoldersRow` message.

`parent_id` = 0 means that the folder is in the root of the collection.

| Field | Type | Description |
| --- | --- | --- |
| `folder_id` | `int64` | Unique identifier of the folder. |
| `parent_id` | `int64` | Identifier of the parent folder. |
| `name` | `string` | Name of the folder. |
| `folder_type` | `int32` | Type of the folder. |
| `avatar_picture_id` | `optional uint64` | Identifier of the avatar picture. |

### `music_items.pb`

Contains metadata about music items in the collection.
Rows are encoded as `MusicItemsRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `music_item_id` | `int64` | Unique identifier of the music item. |
| `name` | `string` | Name of the music item. |
| `folder_id` | `int64` | Identifier of the folder this music item belongs to. |

### `lyrics_items.pb`

Stores lyrics associated with music items.
Rows are encoded as `LyricsItemsRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `lyrics_item_id` | `int64` | Unique identifier of the lyrics item. |
| `music_item_id` | `int64` | Identifier of the associated music item. |
| `lang_code` | `string` | Language code of the lyrics. |

### `picture_items.pb`

Describes images associated with folders or music items.
Rows are encoded as `PictureItemsRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `picture_item_id` | `int64` | Unique identifier of the picture item. |
| `extension` | `string` | File extension of the picture. |
| `folder_id` | `int64` | Identifier of the folder this picture belongs to. |

### `tags.pb`

Manages tags associated with music items and folders.
Rows are encoded as `TagsRow` message.
Only one of `music_item_id` or `folder_id` should be set.

| Field | Type | Description |
| --- | --- | --- |
| `tag_id` | `int64` | Unique identifier of the tag. |
| `music_item_id` | `optional int64` | Identifier of the associated music item. |
| `folder_id` | `optional int64` | Identifier of the associated folder. |
| `tag_name` | `string` | Name of the tag. |
| `tag_value` | `string` | Value of the tag. |

### `music_src_files.pb`

Contains paths to source music files.
Rows are encoded as `MusicSrcFilesRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `src_file_id` | `int64` | Unique identifier of the source file. |
| `music_item_id` | `int64` | Identifier of the associated music item. |
| `source_type` | `int32` | Type of the source |
| `path` | `string` | Path to the source file. |

### `playlists.pb`

Describes playlists in the collection.
Rows are encoded as `PlaylistsRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `playlist_id` | `int64` | Unique identifier of the playlist. |
| `name` | `string` | Name of the playlist. |
| `avatar_picture_id` | `optional int64` | Identifier of the avatar picture associated with the playlist. |

### `playlist_items.pb`

Handles items in playlists.
Rows are encoded as `PlaylistItemsRow` message.

| Field | Type | Description |
| --- | --- | --- |
| `playlist_item_id` | `int64` | Unique identifier of the playlist item. |
| `playlist_id` | `int64` | Identifier of the associated playlist. |
| `music_item_id` | `optional int64` | Identifier of the associated music item. |

