pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("`Failed to analyze the workspace: {0}")]
    WorkspaceError(#[from] workspace::result::Error),
}
