



use std::borrow::Cow;
use std::ops::Add;

use chrono::{ NaiveDate, Local, Duration };
use futures::future::{ ok, err };
use futures::FutureExt;

use crate::service::remote_access::{ RemoteError, WeatherFuture, unix_time };
use crate::service::configuration::{
    Configuration,
    OpenWeatherMap
};


const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial";
pub const MAX_FORECAST_DAYS: i64 = 16;
const WEEK_DAYS: u8 = 5;

#[derive(Debug)]
pub struct UrlBuilder<'w> {
    web_service: Cow<'w, OpenWeatherMap<'w>>,
    city: Option<&'w str>
}

impl<'w> UrlBuilder<'w> {
    
    pub fn new( configuration: &'w Configuration<'w> ) -> UrlBuilder<'w> {
        UrlBuilder {
            web_service: configuration.openweathermap(),
            city: None
        }
    }

    pub fn city( self, city: &'w str ) -> UrlBuilder<'w> {
        UrlBuilder {
            web_service: self.web_service,
            city: Some( city )
        }
    }

    fn forecast( &self, day: NaiveDate, count_day: u8 ) -> WeatherFuture< 'w, Cow<'w, str> > {
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
                    if count_day == WEEK_DAYS {
                        let date = unix_time( today );
                        let url = format!("{}&q={}&date={}&cnt={}&appid={}", BASE_URL, city, date, count_day, self.web_service.api_key());
                        Ok( Cow::Owned( url ) )
                    } else {
                        let date = unix_time( target_day );
                        let count_day = 1;
                        let url = format!("{}&q={}&date={}&cnt={}&appid={}", BASE_URL, city, date, count_day, self.web_service.api_key());
                        Ok( Cow::Owned( url ) )
                    }
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

    pub fn daily( &self, day: NaiveDate ) -> WeatherFuture< 'w, Cow<'w, str> > {
        self.forecast( day, 1 )
    }

    pub fn weekly( &self ) -> WeatherFuture< 'w, Cow<'w, str> > {
        let local = Local::today();
        self.forecast( local.naive_local(), WEEK_DAYS )
    }

}
