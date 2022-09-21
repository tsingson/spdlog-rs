use crate::{
    formatter::{
        pattern_formatter::{Pattern, PatternContext},
        Formatter, FullFormatter,
    },
    Record, StringBuf,
};

/// A pattern that writes the fully formatted text of log records into the
/// output.
///
/// This pattern writes the same formatted text as [`FullFormatter`].
#[derive(Clone, Default)]
pub struct Full {
    full_formatter: FullFormatter,
}

impl Pattern for Full {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> crate::Result<()> {
        self.full_formatter.format(record, dest)?;
        Ok(())
    }
}
