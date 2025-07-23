mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum AppError {
        IoError(std::io::Error),
        ParseError(String),
        Other(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::IoError(err) => write!(f, "IO Error: {err}"),
                AppError::ParseError(msg) => write!(f, "Parse Error: {msg}"),
                AppError::Other(msg) => write!(f, "Error: {msg}"),
            }
        }
    }

    impl std::error::Error for AppError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AppError::IoError(err) => Some(err),
                _ => None,
            }
        }
    }

    impl From<std::io::Error> for AppError {
        fn from(err: std::io::Error) -> AppError {
            AppError::IoError(err)
        }
    }
}
