#![stable(feature = "env", since = "1.0.0")]

use ffi::{OsStr, OsString};
use path::PathBuf;

#[stable(feature = "env", since = "1.0.0")]
pub fn home_dir() -> Option<PathBuf> {
    None
}

#[stable(feature = "env", since = "1.0.0")]
pub fn var<K: AsRef<OsStr>>(_key: K) -> Result<String, ()> {
    Err(())
}

#[stable(feature = "env", since = "1.0.0")]
pub fn var_os<K: AsRef<OsStr>>(_key: K) -> Option<OsString> {
    None
}
