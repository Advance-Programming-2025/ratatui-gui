use log::{Log, Metadata, Record};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Una struttura condivisa per memorizzare i log
pub struct LogBuffer {
    pub logs: Mutex<VecDeque<(log::Level, String)>>,
    pub max_size: usize,
}

impl LogBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            logs: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
        }
    }
}

// Implementazione del tratto Log per intercettare i messaggi
pub struct SimpleTuiLogger {
    pub buffer: Arc<LogBuffer>,
}

impl Log for SimpleTuiLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut logs = self.buffer.logs.lock().unwrap();
            if logs.len() >= self.buffer.max_size {
                logs.pop_front();
            }
            let message = format!("{}", record.args());
            // Qui catturiamo la stringa generata da LogEvent::emit()
            logs.push_back((record.level(), message));
        }
    }
    fn flush(&self) {}
}
