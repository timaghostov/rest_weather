


use chrono::NaiveDate;
use serde_json::Value;

use crate::model::{ Fahrenheit, Forecast };
use crate::model::openweathermap::Response;
use crate::service::remote_access::RemoteError;



pub struct Parser;

impl Parser {
    
    pub fn parse_daily( json: Value, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError > {
        // println!("parse_daily :: {:#?}", json);
        let response: Response = serde_json::from_value::<Response>( json ).map_err( | e | RemoteError::SerdeError( e ) )?;
        response.daily( target_day )
    }

    pub fn parse_weekly( json: Value ) -> Result< Vec< Forecast<Fahrenheit> >, RemoteError > {
        // println!("parse_weekly :: {:#?}", json);
        let response: Response = serde_json::from_value::<Response>( json ).map_err( | e | RemoteError::SerdeError( e ) )?;
        response.weekly()
    }

}