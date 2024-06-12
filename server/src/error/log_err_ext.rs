use tracing::error;

pub trait LogErrExt<T> {
    fn log_error(self) -> Result<T, anyhow::Error>;
}

impl<T> LogErrExt<T> for Result<T, anyhow::Error> {
    fn log_error(self) -> Result<T, anyhow::Error> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }
}
