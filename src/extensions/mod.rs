pub trait ResultExtensions {
    fn log_on_error(&self, message: &str);
}

impl<T, TError> ResultExtensions for Result<T, TError> {
    fn log_on_error(&self, message: &str) {
        match self {
            Err(_error) => {
                println!("{}", message);
            },
            _ => {}
        }
    }
}