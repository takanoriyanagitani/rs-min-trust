#[non_exhaustive]
pub enum Error {
    UnableToParseWasm(String),
    UnableToCreateInstance(String),
    MemoryNotFound(String),
    InvalidOffset(String),
    MainFuncMisbehave(String),
    TrustedFuncError(String),
    UnableToGetFunc(String),
    UnableToCallFunc(String),
    UnableToCopyBytes(String),
    UnableToCopyFromWasm(String),
}
