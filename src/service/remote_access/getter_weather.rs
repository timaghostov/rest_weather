




use std::future::Future;

use std::error::Error;

use chrono::NaiveDate;

use crate::service::configuration::Configuration;
use crate::model::{ Fahrenheit, Forecast };
use super::WeatherFuture;



pub trait GetterWeather {
    
    fn daily_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str, day: NaiveDate ) -> WeatherFuture< 'w, Forecast<Fahrenheit> >;

    fn weekly_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str ) -> WeatherFuture< 'w, Vec< Forecast<Fahrenheit> > >;

}