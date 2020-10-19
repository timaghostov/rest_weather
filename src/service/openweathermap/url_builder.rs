

#![allow(unused_variables, dead_code)]


use std::borrow::Cow;
use std::ops::Add;

use chrono::{ NaiveDate, NaiveDateTime, Local, Duration, Datelike };
use futures::future::{ ok, err };
use futures::FutureExt;

use crate::service::remote_access::{ RemoteError, WeatherFuture, unix_time };
use crate::service::configuration::{
    Configuration,
    OpenWeatherMap
};


const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial";
const MAX_FORECAST_DAYS: i64 = 16;
const WEEK_DAYS: u8 = 5;

#[derive(Debug)]
pub struct UrlBuilder<'a, 'w> {
    web_service: Cow<'a, OpenWeatherMap<'w>>,
    city: Option<&'a str>
}

impl<'a, 'w> UrlBuilder<'a, 'w> {
    
    pub fn new( configuration: &'w Configuration<'w> ) -> UrlBuilder<'a, 'w> {
        UrlBuilder {
            web_service: configuration.openweathermap(),
            city: None
        }
    }

    pub fn city( self, city: &'a str ) -> UrlBuilder<'a, 'w> {
        UrlBuilder {
            web_service: self.web_service,
            city: Some( city )
        }
    }

    fn forecast( &self, day: NaiveDate, count_day: u8 ) -> WeatherFuture< Cow<'static, str> > {
        let result = match self.city {
            Some(city) => {
                let today = Local::today().naive_local();
                let target_day = day.add( Duration::days( ( count_day - 1 ) as i64 ) );
                let max_day = today.add( Duration::days( MAX_FORECAST_DAYS - 1 ) );
                if target_day > max_day {
                    Err( RemoteError::BiggestDay )
                } else if target_day < today {
                    Err( RemoteError::SmallestDay )
                } else {
                    let date = unix_time( target_day );

                    let url = format!("{}&q={}&date={}&cnt={}&appid={}", BASE_URL, city, date, count_day, self.web_service.api_key());
                    Ok( Cow::Owned( url ) )
                }
            },
            None => Err( RemoteError::EmptyCity ),
        };
        let fut = match result {
            Ok( value ) => ok( value ).left_future(),
            Err( error ) => err( error ).right_future(),
        };
        FutureExt::boxed( fut )
    }

    pub fn daily( &self, day: NaiveDate ) -> WeatherFuture< Cow<'static, str> > {
        self.forecast( day, 1 )
    }

    pub fn weekly( &self ) -> WeatherFuture< Cow<'static, str> > {
        let local = Local::today();
        self.forecast( local.naive_local(), WEEK_DAYS )
    }

}
