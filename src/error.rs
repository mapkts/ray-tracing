//! Error types, basically [`Result`] and [`ErrorKind`].
use thiserror::Error;

use std::io::Error as IoError;
use std::result::Result as StdResult;

/// A type alias for [`Result`]<T, [`enum@ErrorKind`]>.
///
/// [`Result`]: std::result::Result
pub type Result<T> = StdResult<T, ErrorKind>;

/// The concrete type of an error.
#[derive(Error, Debug)]
pub enum ErrorKind {
    /// Cannot write pixel color values into a writer.
    #[error("cannot write pixel colors into `{0}`")]
    WriteColor(String),

    /// Represents an [`I/O error`].
    ///
    /// [`I/O error`]: std::io::Error
    #[error(transparent)]
    Io(#[from] IoError),
}
