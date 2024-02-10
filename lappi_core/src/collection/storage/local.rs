use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::ops::Add;
use crate::database_api::{DbExporter, DbImporter, DbTableExporter, DbValue};

pub struct CsvFileDbTableExporter {
    file: File,
}

impl DbTableExporter for CsvFileDbTableExporter {
    fn add_row(&mut self, data: Vec<DbValue>) {
        let list: Vec<String> = data.iter().map(|v| {
            let res = match v {
                DbValue::String(s) => s.clone(),
                DbValue::Number(i) => i.to_string()
            };
            return res;
        }).collect();
        let row_string = list.join("|").add("\n");
        self.file.write_all(row_string.as_bytes()).unwrap();
    }

    fn flush(&mut self) {
        self.file.flush().unwrap();
    }
}

pub struct CsvFileDbExporter {
    root_path: PathBuf,
}

impl DbExporter for CsvFileDbExporter {
    fn get_table_exporter(&self, table_name: &str, columns: Vec<String>) -> Box<dyn DbTableExporter> {
        let path = self.root_path.join(table_name.to_string() + ".csv");
        let mut file = File::create(path).unwrap();
        let header = columns.join("|").add("\n");
        file.write_all(header.as_bytes()).unwrap();
        return Box::new(CsvFileDbTableExporter {
            file
        });
    }
}

impl CsvFileDbExporter {
    pub fn create(ws_path: PathBuf) -> Self {
        return Self {
            root_path: ws_path.join(".collection")
        };
    }
}

pub struct CsvFileDbImporter {
    root_path: PathBuf,
}

impl DbImporter for CsvFileDbImporter {
    fn get_table_rows(&self, table_name: &str, columns: Vec<(String, String)>) -> Vec<Vec<DbValue>> {
        let path = self.root_path.join(table_name.to_string() + ".csv");
        if !path.exists() {
            return Vec::new();
        }
        let file = File::open(path).unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .has_headers(true)
            .from_reader(file);
        let header = reader.headers().unwrap();
        if header.len() != columns.len() {
            panic!("Invalid number of columns in table {}", table_name);
        }
        let mut sorted_columns = Vec::new();
        for (column_name, column_type) in columns {
            let index_in_csv = header.iter().position(|v| v == column_name).unwrap();
            sorted_columns.push((index_in_csv, column_type));
        }
        let reader = reader.records();
        let mut result = Vec::new();
        for row in reader {
            let row = row.unwrap();
            let mut row_data = Vec::new();
            for (index_in_csv, column_type) in &sorted_columns {
                let value = match column_type.as_str() {
                    "INTEGER" => DbValue::Number(row.get(*index_in_csv).unwrap().parse::<i64>().unwrap()),
                    "TEXT" => DbValue::String(row.get(*index_in_csv).unwrap().to_string()),
                    _ => panic!("Unknown column type: {}", column_type),
                };
                row_data.push(value);
            }
            result.push(row_data);
        }
        return result;
    }
}

impl CsvFileDbImporter {
    pub fn create(ws_path: PathBuf) -> Self {
        return Self {
            root_path: ws_path.join(".collection")
        };
    }
}