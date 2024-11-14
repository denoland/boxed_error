# boxed_error

Experimental opinionated way to easily box errors.

Before:

```rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DenoResolveErrorKind {
  #[error("Importing ...")]
  InvalidVendorFolderImport,
  #[error(transparent)]
  MappedResolution(#[from] MappedResolutionError),
  // ...
}

impl DenoResolveErrorKind {
  pub fn into_box(self) -> DenoResolveError {
    DenoResolveError(Box::new(self))
  }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct DenoResolveError(pub Box<DenoResolveErrorKind>);

impl DenoResolveError {
  pub fn as_kind(&self) -> &DenoResolveErrorKind {
    &self.0
  }

  pub fn into_kind(self) -> DenoResolveErrorKind {
    *self.0
  }
}

impl<E> From<E> for DenoResolveError
where
  DenoResolveErrorKind: From<E>,
{
  fn from(err: E) -> Self {
    DenoResolveError(Box::new(DenoResolveErrorKind::from(err)))
  }
}
```

After:

```rs
use boxed_error::boxed;
use thiserror::Error;

#[derive(Debug, Error)]
#[boxed]
pub enum DenoResolveErrorKind {
  #[error("Importing ...")]
  InvalidVendorFolderImport,
  #[error(transparent)]
  MappedResolution(#[from] MappedResolutionError),
  // ...
}
```
