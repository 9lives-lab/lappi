use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::Result;
use camino::Utf8Path;
use num_traits::FromPrimitive;
use protobuf::EnumOrUnknown;
use rusqlite::{Connection, OptionalExtension, params, Rows, ToSql};
use rusqlite::types::FromSql;
use amina_core::events::EventEmitter;
use amina_core::service::{Context, Service};

use crate::collection::OnCollectionUpdated;

struct BatchContext {
    events_emitter: Service<EventEmitter>,
    batch: bool,
    event: OnCollectionUpdated,
}

impl BatchContext {
    fn reset(&mut self) {
        self.event = OnCollectionUpdated::default();
        self.batch = false;
    }

    pub fn on_collection_updated(&mut self) {
        if !self.batch {
            self.events_emitter.emit_event(&self.event);
            self.reset();
        }
    }

}

pub struct DatabaseContext {
    connection: Connection,
    batch_context: BatchContext,
}

impl DatabaseContext {
    pub fn on_folders_updated(&mut self) {
        self.batch_context.event.folders_updated = true;
        self.batch_context.on_collection_updated();
    }

    pub fn on_music_updated(&mut self) {
        self.batch_context.event.music_updated = true;
        self.batch_context.on_collection_updated();
    }

    pub fn on_playlists_updated(&mut self) {
        self.batch_context.event.music_updated = true;
        self.batch_context.on_collection_updated();
    }

    pub fn start_batch(&mut self) {
        log::debug!("start_batch");
        self.batch_context.batch = true;
    }

    pub fn stop_batch(&mut self) {
        log::debug!("stop_batch");
        self.batch_context.events_emitter.emit_event(&self.batch_context.event);
        self.batch_context.reset();
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn is_empty(&self, table_name: &str) -> bool {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let rows_num = self.connection.query_row(
            &query, 
            params![], 
            |row| row.get::<_, i64>(0)
        ).unwrap();
        return 0 == rows_num;
    }

    pub fn set_field_value<T: ToSql>(&self, row_id: i64, table_name: &str, field_name: &str, value: T) -> Result<()>  {
        let query = format!("UPDATE {} SET {} = ?1 WHERE id = ?2", table_name, field_name);
        self.connection.execute(&query, params![value, row_id])?;
        Ok(())
    }

    pub fn get_field_value<T: FromSql>(&self, row_id: i64, table_name: &str, field_name: &str) -> Result<T> {
        let query = format!("SELECT {} FROM {} WHERE id=(?1)", field_name, table_name);
        let result = self.connection.query_row(&query, params![row_id], |row| row.get::<_, T>(0))?;
        Ok(result)
    }

    pub fn get_field_by_key<K: ToSql, T: FromSql>(&self, table_name: &str, key_name: &str, key_value: K, field_name: &str) -> Result<T> {
        let query = format!("SELECT {} FROM {} WHERE {}=(?1)", field_name, table_name, key_name);
        let result = self.connection.query_row(&query, params![key_value], |row| row.get::<_, T>(0))?;
        Ok(result)
    }

    pub fn add_empty_row(&self, table_name: &str) -> Result<i64> {
        let query = format!("INSERT INTO {} DEFAULT VALUES", table_name);
        self.connection.execute(&query, [])?;
        Ok(self.connection.last_insert_rowid())
    }

    pub fn remove_row(&self, table_name: &str, row_id: i64) -> Result<()> {
        let query = format!("DELETE FROM {} WHERE id=(?1)", table_name);
        self.connection.execute(&query, params![row_id])?;
        Ok(())
    }

    pub fn find_or_add_string_row(&self, table_name: &str, field_name: &str, value: &str) -> Result<i64> {
        let query = format!("SELECT id FROM {} WHERE {}=(?1)", table_name, field_name);
        let result = self.connection.query_row(
            &query,
            params![value],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        let row_id = match result {
            Some(id) => id,
            None => {
                let query = format!("INSERT INTO {} ({}) VALUES (?1)", table_name, field_name);
                self.connection.execute(
                    &query,
                    params![value],
                )?;
                self.connection.last_insert_rowid()
            }
        };
        Ok(row_id)
    }

    pub fn find_connection(&self, table_name: &str, from_name: &str, from_id: i64, to_name: &str, to_id: i64) -> Result<bool> {
        let query = format!("SELECT id FROM {} WHERE {}=(?1) AND {}=(?2)", table_name, from_name, to_name);
        let result = self.connection.query_row(
            &query,
            params![from_id, to_id],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result.is_some())
    }

    pub fn add_connection(&self, table_name: &str, from_name: &str, from_id: i64, to_name: &str, to_id: i64) -> Result<()> {
        if !self.find_connection(table_name, from_name, from_id, to_name, to_id)? {
            let query = format!("INSERT INTO {} ({}, {}) VALUES (?1, ?2)", table_name, from_name, to_name);
            self.connection.execute(
                &query,
                params![from_id, to_id],
            )?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn collect_rows(rows: &mut Rows) -> Result<Vec<i64>> {
        let mut id_list = Vec::new();
        while let Some(row) = rows.next()? {
            id_list.push(row.get(0)?);
        }
        Ok(id_list)
    }

    pub fn get_rows_list(&self, table_name: &str) -> Result<Vec<i64>> {
        let query = format!("SELECT id FROM {}", table_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([])?;
        Self::collect_rows(&mut rows)
    }

    pub fn get_fields_list_by_field_i64_value(&self, table_name: &str, return_field_name: &str, field_name: &str, value: i64) -> Result<Vec<i64>> {
        let query = format!("SELECT {} FROM {} WHERE {}=(?1)", return_field_name, table_name, field_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([value])?;
        Self::collect_rows(&mut rows)
    }

    pub fn get_table_info(&self, table_name: &str) -> Result<Vec<(String, String)>> {
        let query = format!("PRAGMA table_info({})", table_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            let name: String = row.get(1)?;
            let column_type: String = row.get(2)?;
            result.push((name, column_type));
        }
        Ok(result)
    }
}

#[derive(Clone)]
pub struct DatabaseUtils {
    context: Arc<Mutex<DatabaseContext>>,
}

impl<'a> DatabaseUtils {
    pub fn new(context: &Context, connection: Connection) -> Self {
        Self {
            context: Arc::new(Mutex::new(DatabaseContext {
                connection,
                batch_context: BatchContext {
                    batch: false,
                    events_emitter: context.get_service(),
                    event: OnCollectionUpdated::default(),
                },
            })),
        }
    }

    pub fn lock(&'a self) -> MutexGuard<'a, DatabaseContext> {
        self.context.lock().unwrap()
    }
}

pub struct ProtobufImporter {
    file: Option<File>,
}

impl ProtobufImporter {
    pub fn create(base_path: &Utf8Path, file_name: &str) -> Result<Self> {
        log::debug!("Import {}", file_name);

        let file_path = base_path.join(file_name);
        let file = if file_path.is_file() {
            Some(File::open(file_path)?)
        } else {
            None
        };

        Ok(Self {
            file,
        })
    }

    pub fn read_next_row<M>(&mut self) -> Result<Option<M>> 
    where
        M: protobuf::Message 
    {
        if let Some(file) = self.file.as_mut() {
            // Read the length of the next message
            let mut length_bytes = [0u8; 4];
            if file.read_exact(&mut length_bytes).is_err() {
                return Ok(None);
            }
            let length = u32::from_le_bytes(length_bytes) as usize;

            // Read the message bytes
            let mut message_bytes = vec![0u8; length];
            file.read_exact(&mut message_bytes)?;

            // Parse the message
            let message = protobuf::Message::parse_from_bytes(&message_bytes)?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}

pub struct ProtobufExporter {
    file: File,
}

impl ProtobufExporter {
    pub fn create(base_path: &Utf8Path, file_name: &str) -> Result<Self> {
        let file_path = base_path.join(file_name);
        let file = File::create(file_path)?;
        Ok(Self {
            file,
        })
    }

    pub fn write_row<M>(&mut self, message: &M) -> Result<()>
    where
        M: protobuf::Message
    {
        // Serialize the message
        let message_bytes = message.write_to_bytes()?;
        // Write the length of the message
        let length = message_bytes.len() as u32;
        self.file.write_all(&length.to_le_bytes())?;
        // Write the message bytes
        self.file.write_all(&message_bytes)?;
        Ok(())
    }

    pub fn write_rows<M, I>(&mut self, messages: I) -> Result<()>
    where
        M: protobuf::Message,
        I: IntoIterator<Item = rusqlite::Result<M>>
    {
        for message in messages {
            self.write_row(&message?)?;
        }
        Ok(())
    }
}

pub fn parse_enum<T>(value: i32) -> rusqlite::Result<T>
where 
    T: FromPrimitive
{
    match T::from_i32(value) {
        Some(enum_value) => Ok(enum_value),
        None => Err(rusqlite::Error::InvalidParameterName(value.to_string())),
    }
}

pub fn parse_pb_enum<T>(value: i32) -> rusqlite::Result<EnumOrUnknown<T>>
where 
    T: protobuf::Enum
{
    match T::from_i32(value) {
        Some(enum_value) => Ok(EnumOrUnknown::new(enum_value)),
        None => Err(rusqlite::Error::InvalidParameterName(value.to_string())),
    }
}

