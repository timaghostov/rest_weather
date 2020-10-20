


use std::fmt;
use serde::{ Deserialize };
use chrono::{ NaiveDateTime, NaiveDate, Local };
use crate::service::remote_access::RemoteError;
use crate::model::{ Forecast, Fahrenheit };





#[derive(Debug, Deserialize)]
pub struct Temperature {
    #[serde(alias = "min")]
    min: f32,
    #[serde(alias = "max")]
    max: f32
}

#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    #[serde(alias = "dt")]
    date: i64,
    #[serde(alias = "temp")]
    temperature: Temperature
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(alias = "list")]
    daily_forecasts: Vec<DailyForecast>
}

impl From<&DailyForecast> for Forecast<Fahrenheit> {
    
    fn from( value: &DailyForecast ) -> Forecast<Fahrenheit> {
        let forecast_date = NaiveDateTime::from_timestamp( value.date, 0 );
        Forecast::new( forecast_date.date(), value.temperature.min.into(), value.temperature.max.into() )
    }

}

impl Response {

    pub fn daily( &self, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError > {
        let mut forecast_opt = None;

        for forecast in &self.daily_forecasts {
            let forecast_date = NaiveDateTime::from_timestamp( forecast.date, 0 );
            if target_day == forecast_date.date() {
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