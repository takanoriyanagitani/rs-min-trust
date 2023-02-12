#[non_exhaustive]
pub enum Error {
    UnableToParseWasm(String),
    UnableToCreateInstance(String),
    MemoryNotFound(String),
    OffsetFuncMissing(String),
    UnableToGetOffset(String),
    InvalidInputOffset(String),
    InvalidOutputOffset(String),
    UnableToCopyInputBytes(String),
    MainFuncMissing(String),
    MainFuncMisbehave(String),
    TrustedFuncError(String),
    UnableToGetOutputFromTrusted(String),
}
