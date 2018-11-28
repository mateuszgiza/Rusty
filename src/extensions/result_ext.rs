use std::fmt::Display;

pub trait ResultExt<T, TError> {
    fn void(self);
    fn on_success<F>(self, action: F) -> Result<T, TError> where F: FnOnce(&T);
    fn on_error<F>(self, action: F) -> Result<T, TError> where F: FnOnce(&TError);
}

impl<T, TError> ResultExt<T, TError> for Result<T, TError>
where
    TError: Display,
{
    fn void(self) {}

    fn on_success<F>(self, action: F) -> Result<T, TError> where F: FnOnce(&T) {
        match &self {
            Ok(ref o) => action(o),
            _ => {}
        };
        self
    }

    fn on_error<F>(self, action: F) -> Result<T, TError> where F: FnOnce(&TError) {
        match &self {
            Err(ref e) => action(e),
            _ => {}
        };
        self
    }
}
