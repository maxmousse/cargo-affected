pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`cargo_metadata` exited with an error: {0}")]
    CargoMetadata(#[from] cargo_metadata::Error),
}
