use std::fmt;

#[derive(Debug)]
pub enum FpsUnlockerError {
    RegistryKeyNotFound(String),
    RegistryValueNotFound(String),
    RegistryAccessDenied(String),
    JsonParseError(String),
    InvalidFpsValue(String),
    GameNotSupported(String),
    UserInputError(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    WinregError(std::io::Error),
}

impl fmt::Display for FpsUnlockerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FpsUnlockerError::RegistryKeyNotFound(path) => {
                write!(f, "Registry key not found: {}\n\nPossible solutions:\n1. Make sure the game is installed and has been launched at least once\n2. Check if the game is installed in a custom location\n3. Try running the program as administrator", path)
            }
            FpsUnlockerError::RegistryValueNotFound(value) => {
                write!(f, "Registry value not found: {}\n\nPossible solutions:\n1. Launch the game and change graphics settings at least once\n2. Make sure you're using the correct game version\n3. Try resetting game settings to default", value)
            }
            FpsUnlockerError::RegistryAccessDenied(msg) => {
                write!(f, "Registry access denied: {}\n\nSolution: Try running the program as administrator", msg)
            }
            FpsUnlockerError::JsonParseError(msg) => {
                write!(
                    f,
                    "Failed to parse JSON data: {}\n\nThis might indicate corrupted game settings",
                    msg
                )
            }
            FpsUnlockerError::InvalidFpsValue(val) => {
                write!(f, "Invalid FPS value: {}", val)
            }
            FpsUnlockerError::GameNotSupported(game) => {
                write!(f, "Game not supported: {}", game)
            }
            FpsUnlockerError::UserInputError(msg) => {
                write!(f, "User input error: {}", msg)
            }
            FpsUnlockerError::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
            FpsUnlockerError::SerdeError(err) => {
                write!(f, "Serialization error: {}", err)
            }
            FpsUnlockerError::WinregError(err) => {
                write!(f, "Windows registry error: {}", err)
            }
        }
    }
}

impl std::error::Error for FpsUnlockerError {}

impl From<std::io::Error> for FpsUnlockerError {
    fn from(error: std::io::Error) -> Self {
        FpsUnlockerError::IoError(error)
    }
}

impl From<serde_json::Error> for FpsUnlockerError {
    fn from(error: serde_json::Error) -> Self {
        FpsUnlockerError::SerdeError(error)
    }
}

pub type Result<T> = std::result::Result<T, FpsUnlockerError>;
