use std::io::ErrorKind;

#[test]
fn test_boxed_error() {
  #[derive(Debug)]
  #[boxed_error::boxed]
  pub enum MyErrorKind {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
  }

  impl std::fmt::Display for MyErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        MyErrorKind::Io(e) => write!(f, "io error: {}", e),
        MyErrorKind::ParseInt(e) => write!(f, "parse int error: {}", e),
      }
    }
  }

  impl std::error::Error for MyErrorKind {}

  let error =
    MyErrorKind::Io(std::io::Error::new(ErrorKind::NotFound, "File not found"))
      .into_box();
  assert_eq!(error.to_string(), "io error: File not found");
}

#[test]
fn test_named_boxed_error() {
  #[derive(Debug)]
  #[boxed_error::boxed(name = "MyNamedError")]
  pub enum MyNamedBoxedError {
    Io(std::io::Error),
  }

  impl std::fmt::Display for MyNamedBoxedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Self::Io(e) => write!(f, "io error: {}", e),
      }
    }
  }

  impl std::error::Error for MyNamedBoxedError {}

  let error = MyNamedBoxedError::Io(std::io::Error::new(
    ErrorKind::NotFound,
    "File not found",
  ));
  let error: MyNamedError = error.into_box();
  assert_eq!(error.to_string(), "io error: File not found");
}
