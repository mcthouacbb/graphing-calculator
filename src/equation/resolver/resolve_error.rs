#[derive(Debug, Clone)]
pub enum ResolveError {
    UnknownVariable(String),
    UnknownFunction(String),
    IncompleteEquation,
    UnsupportedEquation,
}
