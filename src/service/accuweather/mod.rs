

#![allow(unused_imports, dead_code)]


mod impl_remote_access;
mod url_builder;
mod parser;

pub use impl_remote_access::WeahterGet;


use std::future::Future;
use futures::future::LocalBoxFuture;
use std::error::Error;

use parser::Parser;
use url_builder::UrlBuilder;
use crate::model::{ Fahrenheit, Forecast };
use crate::service::remote_access::{ RemoteAccess, GetterWeather, WeatherFuture };
use crate::service::configuration::Configuration;
use chrono::NaiveDate;


#[derive(Debug)]
pub struct AccuWeather<T: RemoteAccess> {
    raccess: T,
}

impl<T> AccuWeather<T> where T: RemoteAccess {

    pub fn new( raccess: T ) -> Self {
        Self {
            raccess
        }
    }
    
}

impl<T> GetterWeather for AccuWeather<T> where T: RemoteAccess {

    fn daily_weather( &self, configuration: &Configuration<'_>, city: &str, day: NaiveDate ) -> WeatherFuture< Forecast<Fahrenheit> > {
        self.raccess.daily_weather( configuration, city, day )
    }

    fn weekly_weather( &self, configuration: &Configuration<'_>, city: &str ) -> WeatherFuture< Vec< Forecast<Fahrenheit> > > {
        self.raccess.weekly_weather( configuration, city )
    }

}