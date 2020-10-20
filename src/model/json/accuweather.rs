


use std::fmt;
use serde::{ Deserialize, Deserializer, de::{ Error, Visitor, Unexpected } };
use chrono::{ NaiveDateTime, NaiveDate };
use crate::service::remote_access::RemoteError;
use crate::model::{ Forecast, Fahrenheit };


pub struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::NaiveDateTime (NaiveDateTimeVisitor)")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: Error, {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
            Ok( dt ) => Ok( dt ),
            Err( error ) => Err(Error::invalid_value(Unexpected::Str(s), &self)),
        }
    }
}

pub fn from_timestamp<'de, D>(d: D) -> Result<NaiveDateTime, D::Error> where D: Deserializer<'de>, {
    d.deserialize_str(NaiveDateTimeVisitor)
}


#[derive(Debug, Deserialize)]
pub struct TemperatureItem {
    #[serde(alias = "Value")]
    value: f32
}

#[derive(Debug, Deserialize)]
pub struct Temperature {
    #[serde(alias = "Minimum")]
    min: TemperatureItem,
    #[serde(alias = "Maximum")]
    max: TemperatureItem
}

#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    #[serde(deserialize_with = "from_timestamp")]
    #[serde(alias = "Date")]
    date: NaiveDateTime,
    #[serde(alias = "Temperature")]
    temperature: Temperature
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(alias = "DailyForecasts")]
    daily_forecasts: Vec<DailyForecast>
}

impl From<&DailyForecast> for Forecast<Fahrenheit> {
    
    fn from( value: &DailyForecast ) -> Forecast<Fahrenheit> {
        Forecast::new( value.date.date(), value.temperature.min.value.into(), value.temperature.max.value.into() )
    }

}

impl Response {

    pub fn daily( &self, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError > {
        let mut forecast_opt = None;

        for forecast in &self.daily_forecasts {
            if target_day == forecast.date.date() {
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