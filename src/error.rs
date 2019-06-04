use std::fmt;
use std::error;

pub type Result<T> = std::result::Result<T, RedisError>;

#[derive(Debug)]
pub struct RedisError {
    msg: &'static str,
    reason: Option<Box<error::Error>>
}

impl RedisError{
    pub fn new(msg :&'static str)
     ->RedisError{
        RedisError{
            msg: msg,
            reason: None 
        }
    }
    pub fn reason(mut self, reason :Option<Box<error::Error>>) -> RedisError{
        self.reason = reason;
        self
    }
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match &self.reason {
            Some(err) => write!(f, "{} caused by {}", self.msg, err),
            None => write!(f, "{}", self.msg),
        }
    }
}

impl error::Error for RedisError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)>{
        match &self.reason {
            Some(err) => Some(err.as_ref()),
            None => None,
        }
    }
    fn description(&self) -> &str{
        self.msg
    }
}

impl From<std::io::Error> for RedisError{
    fn from(err: std::io::Error) -> RedisError {
        RedisError{
            msg: "",
            reason: Some(Box::new(err))
        }
    }
}