pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownCommand { command: String },
    NoCommand,
    UnknownError,
    WrongNumberOfArguments {
        expected: u8,
        got: u8,
    },
    EmptyDescription,
    IdMustBeNumber { id: String },
    FailedToPersistChanges,
    FailedToAccessPersistedData,
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownCommand { command } => write!(f, "Invalid command {}", command),
            Error::UnknownError => write!(f, "Unexpected error"),
            Error::NoCommand => write!(f, "No command provided"),
            Error::WrongNumberOfArguments { expected, got } => {
                write!(f, "Wrong number of arguments: Expected {}, got {}", expected, got)
            },
            Error::EmptyDescription => write!(f, "Task description cannot be empty"),
            Error::IdMustBeNumber { id } => {
                write!(f, "Expected numeric id, got {}", id)
            },
            Error::FailedToPersistChanges => write!(f, "Could not persist changes"),
            Error::FailedToAccessPersistedData => write!(f, "Could not access persisted data"),
        }
    }
}
