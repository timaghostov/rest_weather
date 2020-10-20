


use serde::de::Error as DeError;
use std::error::Error as StdError;
use std::fmt::{ self, Display };
use std::io::Error as IoError;
use toml::de::Error as TomlError;



#[derive( Debug )]
pub enum ConfigurationError {
    IoError( IoError ),
    TomlError( TomlError ),
}

impl Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationError::IoError( error ) => formatter.write_str(&error.to_string()),
            ConfigurationError::TomlError( error ) => formatter.write_str(&error.to_string()),
        }
    }
}

impl StdError for ConfigurationError {}

impl DeError for ConfigurationError {
    fn custom<T: Display>(msg: T) -> Self {
        ConfigurationError::TomlError( TomlError::custom(msg) )
    }
}

impl From<IoError> for ConfigurationError {
    
    fn from(error: IoError) -> ConfigurationError {
        ConfigurationError::IoError( error )
    }

}

impl From<TomlError> for ConfigurationError {
    
    fn from(error: TomlError) -> ConfigurationError {
        ConfigurationError::TomlError( error )
    }

}