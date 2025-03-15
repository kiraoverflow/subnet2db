
use log::{info, warn, error}; // Import log macros

pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger{}
    }
    // Logs an info message
    pub fn info(&self, message: String) {
        info!("INFO: {}", &message);
    }

    // Logs a warning message
    pub fn warn(&self, message: String) {
        warn!("WARNING: {}", &message);
    }

    // Logs an error message
    pub fn error(&self, message: String) {
        error!("ERROR: {}", &message);
    }
}
