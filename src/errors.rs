use thiserror::Error;

#[derive(Debug, Error)]
pub enum Log {
    #[error("Invalid time format specified.")]
    InvalidTimeFormat(#[source] time::error::InvalidFormatDescription)
}


#[derive(Debug, Error)]
pub enum System {
    #[error("Failed to get `which` command out path.")]
    WhichFailed,
    #[error("Command execution failed.")]
    CommandFailed(#[source] std::io::Error),
    #[error("Command '{0}' exited with non-zero status")]
    CommandExitError(String),
    #[error("Failed to found system variable!")]
    EnvVarError(#[source] std::env::VarError)
}

#[derive(Debug, Error)]
pub enum FileSystem {
    #[error("Failed to found file.")]
    FileNotFound(#[source] std::io::Error),
    #[error("Failed to write into file.")]
    FileWriteError(#[source] std::io::Error)
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Log(#[from] Log),
    #[error(transparent)]
    System(#[from] System),
    #[error(transparent)]
    FileSystem(#[from] FileSystem)
}
