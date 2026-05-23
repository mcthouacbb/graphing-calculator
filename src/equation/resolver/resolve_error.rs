#[derive(Debug, Clone)]
pub enum ResolveError {
    UnknownIdentifier(String),
    IncompleteEquation,
}
