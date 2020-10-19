


use std::borrow::Cow;
use chrono::NaiveDate;

use super::remote_access::{ GetterWeather, WeatherFuture, MeasureError };
use super::util::{ average_daily, average_weekly };
use super::configuration::Configuration;
use crate::model::{ Fahrenheit, Forecast };
use crate::rest::RestResponse;



pub struct Service<'a> {
    configuration: Configuration<'a>,
    remote_services: Vec< Box< dyn GetterWeather > >
}

impl<'a> Service<'a> {
    
    pub fn new(configuration: Configuration<'a>) -> Service<'a> {
        Self {
            configuration,
            remote_services: vec![]
        }
    }

    pub fn add_remote_service( &mut self, remote_service: Box< dyn GetterWeather > ) {
        self.remote_services.push( remote_service );
    }

    pub async fn forecast_daily<T: Clone + From<Fahrenheit>>( &self, city: &str, day: NaiveDate ) -> RestResponse<'a, T> {
        let futs: Vec< WeatherFuture< Forecast<Fahrenheit> > > = self.remote_services.iter().map( | rs | rs.daily_weather( &self.configuration, city, day ) ).collect();
        let results = futures::future::join_all( futs ).await;

        // println!("webservice forecast_daily :: {:?}", results);

        let res = average_daily( day, results );
        let res: RestResponse<'a, Fahrenheit> = res.into();
        res.cast_to::<T>()
    }

    pub async fn forecast_weekly<T: Clone + From<Fahrenheit>>( &self, city: &str ) -> RestResponse<'a, T> {
        let futs: Vec< WeatherFuture< Vec< Forecast<Fahrenheit> > > > = self.remote_services.iter().map( | rs | rs.weekly_weather( &self.configuration, city ) ).collect();
        let results = futures::future::join_all( futs ).await;

        // println!("forecast_weekly :: {:?}", results);

        let res = average_weekly( results );
        let res: RestResponse<'a, Fahrenheit> = res.into();
        res.cast_to::<T>()
    }

}