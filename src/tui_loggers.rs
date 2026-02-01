use log::{Log, Metadata, Record};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Shared structure for storing log messages in a circular buffer
pub struct LogBuffer {
    /// Thread-safe deque containing log level and message pairs
    pub logs: Mutex<VecDeque<(log::Level, String)>>,
    /// Maximum number of log entries to keep
    pub max_size: usize,
}

impl LogBuffer {
    /// Creates a new LogBuffer with the specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            logs: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
        }
    }
}

/// Custom logger implementation that captures log messages into a shared buffer
///
/// This logger intercepts all log messages and stores them in a LogBuffer
/// for display in the TUI
pub struct SimpleTuiLogger {
    /// Shared reference to the log buffer
    pub buffer: Arc<LogBuffer>,
}

impl Log for SimpleTuiLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut logs = self.buffer.logs.lock().unwrap();

            // Remove oldest entry if at capacity
            if logs.len() >= self.buffer.max_size {
                logs.pop_front();
            }

            let message = format!("{}", record.args());
            logs.push_back((record.level(), message));
        }
    }

    fn flush(&self) {}
}
