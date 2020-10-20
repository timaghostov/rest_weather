



use std::borrow::Cow;
use std::ops::Add;

use serde_json::Value;
use chrono::{ NaiveDate, Local, Duration };
use futures::future::{ ok, err };
use futures::FutureExt;

use crate::service::remote_access::{ RemoteError, WeatherFuture, get_response };
use crate::service::configuration::{
    Configuration,
    AccuWeather
};


const LOCATION_BASE_URL: &str = "http://dataservice.accuweather.com/locations/v1/cities/search?";
//http://api.accuweather.com/forecasts/v1/daily/1day/349084?apikey={your key}
const BASE_URL_TODAY: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/1day/";
//http://api.accuweather.com/forecasts/v1/daily/15day/349084?apikey={your key}
const BASE_URL_ON_DAY: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/5day/";
//http://api.accuweather.com/forecasts/v1/daily/5day/349084?apikey={your key}
const BASE_URL_ON_WEEK: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/5day/";
pub const MAX_FORECAST_DAYS: i64 = 5;
const WEEK_DAYS: u8 = 5;

#[derive(Debug)]
pub struct UrlBuilder<'w> {
    web_service: Cow<'w, AccuWeather<'w>>,
    city: Option<Cow<'w, str>>
}

impl<'w> UrlBuilder<'w> {
    
    pub fn new( configuration: &'w Configuration<'w> ) -> UrlBuilder<'w> {
        UrlBuilder {
            web_service: configuration.accuweather(),
            city: None
        }
    }

    pub fn city( self, city: &'w str ) -> UrlBuilder<'w> {
        UrlBuilder {
            web_service: self.web_service,
            city: Some( Cow::Borrowed( city ) )
        }
    }

    fn get_location_key<'b>( json: Value ) -> Result< Cow<'b, str>, RemoteError > {
        let location_opt = json.as_array()
                        .and_then( | array | {
                            if array.len() > 0 {
                                array[0].as_object()
                                    .and_then( | obj | {
                                        obj.get("Key").and_then( | value | value.as_str() )
                                    } )
                            } else {
                                None
                            }
                        } );
        match location_opt {
            Some( location ) => {
                Ok( Cow::Owned( location.to_string() ) )
            },
            None => Err( RemoteError::EmptyCity ),
        }
    }

    fn fut_location_key( &self ) -> WeatherFuture< 'w, Cow<'w, str> > {
        let fut = match self.city.to_owned() {
            Some( city ) => {
                let url = format!("{}q={}&apikey={}", LOCATION_BASE_URL, city, self.web_service.api_key());
                // println!("fut_location_key :: {:?}", url);
                let fut = get_response( Cow::Owned( url ) )
                            .then( | value_res | {                                
                                match value_res.and_then( | value | Self::get_location_key( value ) ) {
                                    Ok( value ) => {
                                        ok( value ).left_future()
                                    },
                                    Err( error ) => err( error ).right_future(),
                                }
                            } );
                fut.left_future()
            },
            None => err( RemoteError::EmptyCity ).right_future(),
        };
        FutureExt::boxed( fut )
    }

    fn forecast( &self, day: NaiveDate, count_day: u8 ) -> WeatherFuture< 'w, Cow<'w, str> > {
        let api_key: String = self.web_service.api_key().into_owned();
        let fut = self.fut_location_key()
                    .then( move | location_key_res | {
                        // println!("forecast location_key_res :: {:?}", location_key_res);
                        match location_key_res {
                            Ok( location_key ) => {
                                let today = Local::today().naive_local();
                                let target_day = day.add( Duration::days( ( count_day - 1 ) as i64 ) );
                                let max_day = today.add( Duration::days( MAX_FORECAST_DAYS - 1 ) );
                                let url_result =    if target_day > max_day {
                                                    Err( RemoteError::BiggestDay )
                                                } else if target_day < today {
                                                    Err( RemoteError::SmallestDay )
                                                } else {
                                                    if today == target_day {
                                                        if count_day == 1 {//on today
                                                            //http://api.accuweather.com/forecasts/v1/daily/1day/349084?apikey={your key}
                                                            // const BASE_URL_TODAY: &str = "http://api.accuweather.com/forecasts/v1/daily/1day/";
                                                            let url = format!("{}{}?apikey={}", BASE_URL_TODAY, location_key, api_key);
                                                            Ok( Cow::Owned( url ) )
                                                        } else {//on week
                                                            //http://api.accuweather.com/forecasts/v1/daily/5day/349084?apikey={your key}
                                                            // const BASE_URL_ON_WEEK: &str = "http://api.accuweather.com/forecasts/v1/daily/5day/";
                                                            let url = format!("{}{}?apikey={}", BASE_URL_ON_WEEK, location_key, api_key);
                                                            Ok( Cow::Owned( url ) )
                                                        }
                                                    } else {
                                                        //http://api.accuweather.com/forecasts/v1/daily/15day/349084?apikey={your key}
                                                        // const BASE_URL_ON_DAY: &str = "http://api.accuweather.com/forecasts/v1/daily/15day/";
                                                        let url = format!("{}{}?apikey={}", BASE_URL_ON_DAY, location_key, api_key);
                                                        Ok( Cow::Owned( url ) )
                                                    }
                                                };
                                let fut = match url_result {
                                    Ok( url ) => ok( url ).left_future(),
                                    Err( error ) => err( error ).right_future(),
                                };
                                fut.left_future()
                            },
                            Err( error ) => err( error ).right_future(),
                        }
                    } );
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
