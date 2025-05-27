
#[allow(dead_code)]
pub type MimiciError = Box<dyn std::error::Error>;

#[allow(dead_code)]
pub type MimiciResult<T> = Result<T, MimiciError>;
