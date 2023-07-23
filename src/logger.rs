use chrono::Utc;

#[derive(Debug)]
pub struct Logger{
    path: String,
    log_data: Vec<String>,
    date: String,
}

impl Logger {
    pub fn initialize(path: String)->Logger{
        Logger{
            path: path,
            log_data: vec![],
            date: chrono::offset::Local::now().to_string(),
        }
    }
}