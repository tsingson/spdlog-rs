//! Provides a basic and default logger.

use std::sync::{Arc, Mutex};

use crate::{
    logger::Logger,
    sink::{Sink, Sinks},
    ErrorHandler, LevelFilter, LogMsg, Metadata, Record,
};

/// A basic and default logger.
pub struct BasicLogger {
    level: LevelFilter,
    sinks: Sinks,
    error_handler: Mutex<Option<ErrorHandler>>,
}

impl BasicLogger {
    /// Constructs a [`BasicLogger`].
    pub fn new() -> BasicLogger {
        BasicLogger {
            level: LevelFilter::Info,
            sinks: vec![],
            error_handler: Mutex::new(None),
        }
    }

    /// Constructs a [`BasicLogger`] with a [`Sink`].
    pub fn with_sink(sink: Arc<dyn Sink>) -> BasicLogger {
        BasicLogger {
            level: LevelFilter::Info,
            sinks: vec![sink],
            error_handler: Mutex::new(None),
        }
    }

    /// Constructs a [`BasicLogger`] with multiple [`Sink`] s.
    pub fn with_sinks<I>(iter: I) -> BasicLogger
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        BasicLogger {
            level: LevelFilter::Info,
            sinks: iter.into_iter().collect(),
            error_handler: Mutex::new(None),
        }
    }
}

impl log::Log for BasicLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        self.sink_record(record);
    }

    fn flush(&self) {}
}

impl Logger for BasicLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn set_level(&mut self, level: LevelFilter) {
        self.level = level;
    }

    fn sinks(&self) -> &Sinks {
        &self.sinks
    }

    fn sinks_mut(&mut self) -> &mut Sinks {
        &mut self.sinks
    }

    fn sink_record(&self, record: &Record) {
        let metadata = record.metadata();
        let log_msg = LogMsg::new(record);

        self.sinks.iter().for_each(|sink| {
            if sink.enabled(metadata) {
                if let Err(err) = sink.log(&log_msg) {
                    if let Some(handler) = self.error_handler.lock().unwrap().as_mut() {
                        handler(err)
                    }
                }
            }
        })
    }

    fn set_error_handler(&mut self, handler: ErrorHandler) {
        self.error_handler.lock().unwrap().replace(handler);
    }
}

impl Default for BasicLogger {
    fn default() -> BasicLogger {
        BasicLogger::new()
    }
}