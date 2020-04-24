use mongodb::error::{ Error as MongoDBError };

/// The Error definition.  Variants can carry payloads, which I am using
/// here to carry the source or causal error.  The variant does not need to
/// have the same name as the foreign error type it wraps.
#[derive(Debug)]
pub enum Error {
  DefaultError(String),
  CursorNotFoundError,
  MongoError(MongoDBError)
}

/// Allows your error to be displayed using `{}`, and not just `{:?}`
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", match self {
          Error::DefaultError(message) => format!("{}", message),
          Error::CursorNotFoundError => format!("MongoDB Error"),
          Error::MongoError(_error) => format!("MongoDB Error"),
      })
  }
}
