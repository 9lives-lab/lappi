use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;

#[derive(Debug, Clone)]
pub enum DbValue {
    Null,
    String(String),
    Number(i64),
}

impl Default for DbValue {
    fn default() -> Self {
        DbValue::Null
    }
}

impl ToSql for DbValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            DbValue::Null => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Null)),
            DbValue::String(s) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(s.clone()))),
            DbValue::Number(n) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(*n))),
        }
    }
}

pub trait DbTableExporter {
    fn add_row(&mut self, data: Vec<DbValue>);
    fn flush(&mut self);
}

pub trait DbExporter {
    fn get_table_exporter(&self, table_name: &str, columns: Vec<String>) -> Box<dyn DbTableExporter>;
}

pub trait DbImporter {
    fn get_table_rows(&self, table_name: &str, columns: Vec<(String, String)>) -> Vec<Vec<DbValue>>;
}

