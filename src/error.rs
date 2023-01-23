use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("Could not find dir \"{0}\"")]
    DailyDirNotFound(String),

    #[error("Today's note already exists!")]
    DailyFileAlreadyExists,

    #[error("JS function \"{0}\" failed")]
    JsFunctionError(&'static str),

    #[error("{0}")]
    Misc(&'static str),
}
