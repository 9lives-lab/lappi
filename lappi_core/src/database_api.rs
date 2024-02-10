use rusqlite::ToSql;

pub mod error {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub enum DbError {
        GenericError(String),
    }

    impl From<rusqlite::Error> for DbError {
        fn from(err: rusqlite::Error) -> DbError {
            DbError::GenericError(err.to_string())
        }
    }

    impl std::fmt::Display for DbError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database error")
        }
    }

    impl std::error::Error for DbError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }
}

pub type DbResult<T, E = error::DbError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub enum DbValue {
    String(String),
    Number(i64),
}

impl ToSql for DbValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            DbValue::String(s) => Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Text(s.clone()))),
            DbValue::Number(n) => Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(*n))),
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

