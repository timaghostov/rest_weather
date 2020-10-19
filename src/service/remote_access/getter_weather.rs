




use std::future::Future;

use std::error::Error;

use chrono::NaiveDate;

use crate::service::configuration::Configuration;
use crate::model::{ Fahrenheit, Forecast };
use super::WeatherFuture;



pub trait GetterWeather {
    
    fn daily_weather( &self, configuration: &Configuration<'_>, city: &str, day: NaiveDate ) -> WeatherFuture< Forecast<Fahrenheit> >;

    fn weekly_weather( &self, configuration: &Configuration<'_>, city: &str ) -> WeatherFuture< Vec< Forecast<Fahrenheit> > >;

}