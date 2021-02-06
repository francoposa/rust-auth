use std::fmt;

pub trait Hasher {
    fn hash(&self, password: String) -> Result<String, HasherError>;
    //fn verify(&self, password: String, hash: String) -> bool;
}

#[derive(Debug)]
pub enum HasherError {
    HasherError { message: String },
}

impl fmt::Display for HasherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            HasherError::HasherError { message } => f.write_str(&message),
        }
    }
}
