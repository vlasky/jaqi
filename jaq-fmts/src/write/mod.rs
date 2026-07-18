//! Write values in different formats.
#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "tabular")]
pub mod tabular;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "xml")]
pub mod xml;
#[cfg(feature = "yaml")]
pub mod yaml;
pub use jaq_json::write as json;

#[cfg(feature = "all")]
mod formats;
#[cfg(feature = "all")]
mod funs;
#[cfg(feature = "all")]
pub use formats::write;
#[cfg(feature = "all")]
pub use funs::funs;

use std::io::{self, IsTerminal, Write};

/// Write options.
#[derive(Default)]
pub struct Writer {
    /// output format
    pub format: crate::Format,
    /// pretty printer
    pub pp: json::Pp,
    /// concatenate outputs without newline
    pub join: bool,
}

/// Run `f` on locked stdout, buffering writes if stdout is not a terminal.
///
/// The closure also receives whether stdout is a terminal, so that
/// it can decide whether to flush after individual outputs.
/// If stdout is not a terminal, then the buffer is flushed at the end,
/// propagating any error that occurs during the flush.
pub fn with_stdout<T, E: From<io::Error>>(
    f: impl FnOnce(&mut dyn Write, bool) -> Result<T, E>,
) -> Result<T, E> {
    let stdout = io::stdout();
    if stdout.is_terminal() {
        f(&mut stdout.lock(), true)
    } else {
        let mut w = io::BufWriter::new(stdout.lock());
        let y = f(&mut w, false)?;
        w.flush()?;
        Ok(y)
    }
}
