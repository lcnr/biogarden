use crate::ds::graph::GraphErr;
use std::error::Error;
use std::fmt;

#[derive(Debug)] // Allow the use of "{:?}" format specifier
pub enum BioError {
    InvalidInputSize,
    ItemNotFound,
    TypeConversionError, 
    GraphError(GraphErr),
}

// Allow the use of "{}" format specifier
impl fmt::Display for BioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BioError::InvalidInputSize => write!(f, "Provided inputs have invalid size!"),
            BioError::ItemNotFound => write!(f, "The requested item does not exist!"),
            BioError::TypeConversionError => write!(f, "The requested type conversion resulted in an error!"),
            BioError::GraphError(ref source) => write!(f, "An error occurred during graph processing! {}", source),
        }
    }
}

// Allow this type to be treated like an error
impl Error for BioError {
    fn source(&self) -> Option<&(dyn Error + 'static)>{
        match self {
            BioError::InvalidInputSize => None,
            BioError::ItemNotFound => None,
            BioError::TypeConversionError => None,
            BioError::GraphError(ref source) => Some(source)
        }
    }
}

impl From<GraphErr> for BioError {
    fn from(cause: GraphErr) -> BioError {
        BioError::GraphError(cause)
    }
}

pub type Result<T> = std::result::Result<T, BioError>;
