use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::{Connection, OptionalExtension, params, Rows, ToSql};
use rusqlite::types::FromSql;
use amina_core::events::EventEmitter;
use amina_core::service::{Context, Service};

use crate::collection::OnCollectionUpdated;
use crate::database::api::{DbExporter, DbImporter, DbResult, DbValue};

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

    pub fn set_field_value<T: ToSql>(&self, row_id: i64, table_name: &str, field_name: &str, value: T) -> DbResult<()>  {
        let query = format!("UPDATE {} SET {} = ?1 WHERE id = ?2", table_name, field_name);
        self.connection.execute(&query, params![value, row_id])?;
        Ok(())
    }

    pub fn get_field_value<T: FromSql>(&self, row_id: i64, table_name: &str, field_name: &str) -> DbResult<T> {
        let query = format!("SELECT {} FROM {} WHERE id=(?1)", field_name, table_name);
        let result = self.connection.query_row(&query, params![row_id], |row| row.get::<_, T>(0))?;
        Ok(result)
    }

    pub fn get_field_by_key<K: ToSql, T: FromSql>(&self, table_name: &str, key_name: &str, key_value: K, field_name: &str) -> DbResult<T> {
        let query = format!("SELECT {} FROM {} WHERE {}=(?1)", field_name, table_name, key_name);
        let result = self.connection.query_row(&query, params![key_value], |row| row.get::<_, T>(0))?;
        Ok(result)
    }

    pub fn add_empty_row(&self, table_name: &str) -> DbResult<i64> {
        let query = format!("INSERT INTO {} DEFAULT VALUES", table_name);
        self.connection.execute(&query, [])?;
        Ok(self.connection.last_insert_rowid())
    }

    pub fn remove_row(&self, table_name: &str, row_id: i64) -> DbResult<()> {
        let query = format!("DELETE FROM {} WHERE id=(?1)", table_name);
        self.connection.execute(&query, params![row_id])?;
        Ok(())
    }

    pub fn find_or_add_string_row(&self, table_name: &str, field_name: &str, value: &str) -> DbResult<i64> {
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

    pub fn find_connection(&self, table_name: &str, from_name: &str, from_id: i64, to_name: &str, to_id: i64) -> DbResult<bool> {
        let query = format!("SELECT id FROM {} WHERE {}=(?1) AND {}=(?2)", table_name, from_name, to_name);
        let result = self.connection.query_row(
            &query,
            params![from_id, to_id],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result.is_some())
    }

    pub fn add_connection(&self, table_name: &str, from_name: &str, from_id: i64, to_name: &str, to_id: i64) -> DbResult<()> {
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

    pub fn collect_rows(rows: &mut Rows) -> DbResult<Vec<i64>> {
        let mut id_list = Vec::new();
        while let Some(row) = rows.next()? {
            id_list.push(row.get(0)?);
        }
        Ok(id_list)
    }

    pub fn get_rows_list(&self, table_name: &str) -> DbResult<Vec<i64>> {
        let query = format!("SELECT id FROM {}", table_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([])?;
        Self::collect_rows(&mut rows)
    }

    pub fn get_fields_list_by_field_i64_value(&self, table_name: &str, return_field_name: &str, field_name: &str, value: i64) -> DbResult<Vec<i64>> {
        let query = format!("SELECT {} FROM {} WHERE {}=(?1)", return_field_name, table_name, field_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([value])?;
        Self::collect_rows(&mut rows)
    }

    pub fn get_table_info(&self, table_name: &str) -> DbResult<Vec<(String, String)>> {
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

    pub fn export_table(&self, table_name: &str, exporter: &dyn DbExporter) -> DbResult<()> {
        let table_info = self.get_table_info(table_name)?;
        let columns: Vec<String> = table_info.iter().map(|(name, _)| name.clone()).collect();
        let query = format!("SELECT {} FROM {}", columns.join(", "), table_name);
        let mut stmt = self.connection.prepare(&query)?;
        let mut rows = stmt.query([])?;
        let mut table_exporter = exporter.get_table_exporter(table_name, columns);
        while let Some(row) = rows.next()? {
            let mut row_data = Vec::new();
            for i in 0..table_info.len() {
                let value = match table_info[i].1.as_str() {
                    "INTEGER" => row.get::<usize, Option<i64>>(i)?.map(|value| DbValue::Number(value)),
                    "TEXT" => row.get::<usize, Option<String>>(i)?.map(|value| DbValue::String(value)),
                    _ => panic!("Unknown column type: {}", table_info[i].1),
                };
                row_data.push(value.unwrap_or_default());
            }
            table_exporter.add_row(row_data);
        }
        table_exporter.flush();
        Ok(())
    }

    pub fn import_table(&self, table_name: &str, importer: &dyn DbImporter) -> DbResult<()> {
        let table_info = self.get_table_info(table_name)?;
        let columns: Vec<String> = table_info.iter().map(|(name, _)| name.clone()).collect();
        let data = importer.get_table_rows(table_name, table_info);
        for row in data {
            let mut s = "?,".repeat(columns.len());
            s.pop();
            let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns.join(", "), s);
            log::trace!("query: {} | {:?}", query, &row);
            self.connection.execute(
                &query,
                rusqlite::params_from_iter(row.into_iter())
            )?;
        }
        Ok(())
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
