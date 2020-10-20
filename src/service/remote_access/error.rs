


use std::error::Error;
use std::fmt;

use reqwest::{ StatusCode, Error as ReqwestError };
use serde_json::Error as SerdeError;




#[derive(Debug, PartialEq)]
pub struct BadStatus {
    status: StatusCode
}

impl BadStatus {
    
    pub fn new(status: StatusCode) -> Self {
        Self {
            status
        }
    }

}

impl fmt::Display for BadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BadStatus ({})", self.status)
    }
}

impl Error for BadStatus {}

#[derive(Debug)]
pub enum RemoteError {
    EmptyCity                       ,
    BiggestDay                      ,
    SmallestDay                     ,
    UnknownResult                   ,
    BadStatus( BadStatus )          ,
    ReqwestError( ReqwestError )    ,
    SerdeError( SerdeError )        ,
}

impl RemoteError {
    
    pub fn is_empty_city( &self ) -> bool {
        match self {
            RemoteError::EmptyCity => true,
            _ => false,
        }
    }

    pub fn is_biggest_day( &self ) -> bool {
        match self {
            RemoteError::BiggestDay => true,
            _ => false,
        }
    }

    pub fn is_smallest_day( &self ) -> bool {
        match self {
            RemoteError::SmallestDay => true,
            _ => false,
        }
    }

    pub fn is_unknown_result( &self ) -> bool {
        match self {
            RemoteError::UnknownResult => true,
            _ => false,
        }
    }

    pub fn is_bad_status( &self ) -> bool {
        match self {
            RemoteError::BadStatus( _ ) => true,
            _ => false,
        }
    }

    pub fn is_reqwest_error( &self ) -> bool {
        match self {
            RemoteError::ReqwestError( _ ) => true,
            _ => false,
        }
    }

    pub fn is_serde_error( &self ) -> bool {
        match self {
            RemoteError::SerdeError( _ ) => true,
            _ => false,
        }
    }

}

impl fmt::Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoteError::EmptyCity => write!(f, "EmptyCity"),
            RemoteError::BiggestDay => write!(f, "BiggestDay"),
            RemoteError::SmallestDay => write!(f, "SmallestDay"),
            RemoteError::UnknownResult => write!(f, "UnknownResult"),
            RemoteError::BadStatus( value ) => write!(f, "BadStatus ({})", value),
            RemoteError::ReqwestError( value ) => write!(f, "ReqwestError ({})", value),
            RemoteError::SerdeError( value ) => write!(f, "SerdeError ({})", value),
        }
        
    }
}

impl Error for RemoteError {}

#[derive(Debug)]
pub enum MeasureError {
    UnknownResult,
    BadDateFormat,
    RemoteError( Vec<RemoteError> )
}

impl MeasureError {

    pub fn has_empty_city( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_empty_city() )
                }
            },
        }
    }

    pub fn has_biggest_day( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_biggest_day() )
                }
            },
        }
    }

    pub fn has_smallest_day( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_smallest_day() )
                }
            },
        }
    }

    pub fn has_unknown_result( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_unknown_result() )
                }
            },
        }
    }

    pub fn has_bad_status( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_bad_status() )
                }
            },
        }
    }

    pub fn has_reqwest_error( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_reqwest_error() )
                }
            },
        }
    }

    pub fn has_serde_error( &self ) -> bool {
        match self {
            MeasureError::UnknownResult => false,
            MeasureError::BadDateFormat => false,
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    false
                } else {
                    errors.iter().any( | x | x.is_serde_error() )
                }
            },
        }
    }

}

impl fmt::Display for MeasureError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeasureError::UnknownResult => write!(f, "Forecast result is unknown."),
            MeasureError::BadDateFormat => write!(f, "Bad format date. Apply format: yyyy-mm-dd."),
            MeasureError::RemoteError( errors ) => {
                if errors.is_empty() {
                    write!(f, "Forecast result is unknown.")
                } else {
                    if self.has_empty_city() {
                        write!(f, "Forecast result is unknown. Check your city name.")
                    } else if self.has_smallest_day() {
                        write!(f, "Forecast result is unknown. Target day is before today. Check your Target day.")
                    } else if self.has_biggest_day() {
                        write!(f, "Forecast result is unknown. Target day is after MAX_DATE ( MAX_DATE for AccuWeather is 5 days, MAX_DATE for OpenWeatherMap is 16 days, MAX_DATE for WeatherBit is 16 days ) day. Check your Target day.")
                    } else if self.has_reqwest_error() {
                        write!(f, "Forecast result is unknown. Bad requests. Check your urls or Net connections.")
                    } else if self.has_serde_error() {
                        write!(f, "Forecast result is unknown. Bad convert results. Check your data structures.")
                    } else if self.has_unknown_result() {
                        write!(f, "Forecast result is unknown. Unknown error.")
                    } else if self.has_bad_status() {
                        write!(f, "Forecast result is unknown. Bad statuses.")
                    } else {
                        write!(f, "Forecast result is unknown.")
                    }
                }
            },
        }
        
    }
}