use lazy_static::*;
use std::io::{Write};
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
}

struct Logger {
    pub output: Box<dyn Write + Send + Sync>
}

/// The type of a log message
pub enum LogType {
    /// Used for providing useful information or notifications.
    Info,
    /// Used for displaying fatal errors or internal problems.
    Error,
    /// Used for providing debug information for developers, usually disabled in production.
    Debug,
}

fn log_type_to_str(log_type: LogType) -> &'static str {
    match log_type {
        LogType::Info => "INFO",
        LogType::Error => "ERR",
        LogType::Debug => "DEBUG"
    }
}

pub fn set_logging_output(output: impl Write + Send + Sync + 'static) {
    LOGGER.lock().unwrap().output = Box::new(output);
}

pub fn info<T>(msg: T) where T: Into<String> {
    LOGGER.lock().unwrap().log(LogType::Info, msg.into()).unwrap();
}

pub fn error<T>(msg: T) where T: Into<String> {
    LOGGER.lock().unwrap().log(LogType::Error, msg.into()).unwrap();
}

#[cfg(build = "debug")]
pub fn debug<T>(msg: T) where T: Into<String> {
    LOGGER.lock().unwrap().log(LogType::Debug, msg.into()).unwrap();
}

#[cfg(build = "release")]
pub fn debug<T>(msg: T) where T: Into<String> {}

pub type LogTypeFilter = Vec<LogType>;

impl Logger {
    fn log(&mut self, log_type: LogType, msg: String) -> Result<(), std::io::Error>{
        let log = format!("[{}]: {}\n", log_type_to_str(log_type), msg);
        self.output.write_all(log.as_bytes())?;
        self.output.flush()?;
        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            output: Box::new(std::io::stdout())
        }
    }
}
