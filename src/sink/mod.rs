//! Provides sinks to flexibly output log messages to specified targets.

pub mod file_sink;
pub mod stderr_sink;
pub mod stderr_style_sink;
pub mod stdout_sink;
pub mod stdout_style_sink;

mod std_out_stream_sink;
mod std_out_stream_style_sink;

pub use file_sink::*;
pub use std_out_stream_style_sink::StyleSink;
pub use stderr_sink::*;
pub use stderr_style_sink::*;
pub use stdout_sink::*;
pub use stdout_style_sink::*;

use std::sync::Arc;

use crate::{formatter::Formatter, LevelFilter, LogMsg, Metadata, Result};

/// A trait for sinks.
///
/// Sinks are the objects that actually write the log to their target. Each sink
/// should be responsible for only single target (e.g file, console, db), and
/// each sink has its own private instance of [`Formatter`] object.
///
/// A [`Logger`] can combine multiple [`Sink`] s.
///
/// [`Logger`]: crate::logger::Logger
pub trait Sink: Sync + Send {
    /// Determines if a log message with the specified metadata would be logged.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level()
    }

    /// Logs the `msg`.
    ///
    /// Internally filtering the log message level is redundant, it should be
    /// filtered by the caller by calling [`Sink::enabled`]. Its implementation
    /// should guarantee that it will never panic even if the caller did not
    /// filter it by calling [`Sink::enabled`], otherwise it should always
    /// filter these potential panic cases internally.
    fn log(&self, msg: &LogMsg) -> Result<()>;

    /// Flushes any buffered records.
    fn flush(&self) -> Result<()>;

    /// Getter of the log filter level.
    fn level(&self) -> LevelFilter;

    /// Setter of the log filter level.
    fn set_level(&mut self, level: LevelFilter);

    /// Getter of the formatter.
    fn formatter(&self) -> &dyn Formatter;

    /// Setter of the formatter.
    fn set_formatter(&mut self, formatter: Box<dyn Formatter>);
}

/// A container for [`Sink`] s.
pub type Sinks = Vec<Arc<dyn Sink>>;

pub(crate) mod macros {
    macro_rules! forward_sink_methods {
        ($struct_type:ident, $inner_name:ident) => {
            use crate::{formatter::Formatter, sink::Sink, LevelFilter, LogMsg, Result};

            impl Sink for $struct_type {
                fn log(&self, msg: &LogMsg) -> Result<()> {
                    self.$inner_name.log(msg)
                }
                fn flush(&self) -> Result<()> {
                    self.$inner_name.flush()
                }
                fn level(&self) -> LevelFilter {
                    self.$inner_name.level()
                }
                fn set_level(&mut self, level: LevelFilter) {
                    self.$inner_name.set_level(level)
                }
                fn formatter(&self) -> &dyn Formatter {
                    self.$inner_name.formatter()
                }
                fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
                    self.$inner_name.set_formatter(formatter)
                }
            }
        };
    }
    pub(crate) use forward_sink_methods;
}