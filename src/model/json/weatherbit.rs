


use std::fmt;
use serde::{ Deserialize, Deserializer, de::{ Error, Visitor, Unexpected } };
use chrono::{ NaiveDate };
use crate::service::remote_access::RemoteError;
use crate::model::{ Forecast, Fahrenheit };


pub struct NaiveDateVisitor;

impl<'de> Visitor<'de> for NaiveDateVisitor {
    type Value = NaiveDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::NaiveDate (NaiveDateVisitor)")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: Error, {
        match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok( dt ) => Ok( dt ),
            Err( _error ) => Err(Error::invalid_value(Unexpected::Str(s), &self)),
        }
    }
}

pub fn from_timestamp<'de, D>(d: D) -> Result<NaiveDate, D::Error> where D: Deserializer<'de>, {
    d.deserialize_str(NaiveDateVisitor)
}


#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    #[serde(deserialize_with = "from_timestamp")]
    #[serde(alias = "datetime")]
    date: NaiveDate,
    #[serde(alias = "min_temp")]
    min: f32,
    #[serde(alias = "max_temp")]
    max: f32
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(alias = "data")]
    daily_forecasts: Vec<DailyForecast>
}

impl From<&DailyForecast> for Forecast<Fahrenheit> {
    
    fn from( value: &DailyForecast ) -> Forecast<Fahrenheit> {
        Forecast::new( value.date, value.min.into(), value.max.into() )
    }

}

impl Response {

    pub fn daily( &self, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError > {
        let mut forecast_opt = None;

        for forecast in &self.daily_forecasts {
            if target_day == forecast.date {
                forecast_opt = Some( forecast.into() );
                break;
            }
        }

        match forecast_opt {
            Some( forecast ) => Ok( forecast ),
            None => Err( RemoteError::UnknownResult ),
        }
    }

    pub fn weekly( &self ) -> Result< Vec<Forecast<Fahrenheit>>, RemoteError > {
        let mut forecasts = vec![];

        for forecast in &self.daily_forecasts {
            forecasts.push( forecast.into() )
        }

        Ok( forecasts )
    }

}