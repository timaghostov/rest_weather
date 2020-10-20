



mod impl_remote_access;
mod url_builder;
mod parser;

pub use impl_remote_access::WeahterGet;

use parser::Parser;
use url_builder::UrlBuilder;
use crate::model::{ Fahrenheit, Forecast };
use crate::service::remote_access::{ RemoteAccess, GetterWeather, WeatherFuture };
use crate::service::configuration::Configuration;
use chrono::NaiveDate;


#[derive(Debug)]
pub struct WeatherBit<T: RemoteAccess> {
    raccess: T,
}

impl<T> WeatherBit<T> where T: RemoteAccess {

    pub fn new( raccess: T ) -> Self {
        Self {
            raccess
        }
    }
    
}

impl<T> GetterWeather for WeatherBit<T> where T: RemoteAccess {

    fn daily_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str, day: NaiveDate ) -> WeatherFuture< 'w, Forecast<Fahrenheit> > {
        self.raccess.daily_weather( configuration, city, day )
    }

    fn weekly_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str ) -> WeatherFuture< 'w, Vec< Forecast<Fahrenheit> > > {
        self.raccess.weekly_weather( configuration, city )
    }

}