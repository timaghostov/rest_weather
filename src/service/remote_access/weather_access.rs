



use std::borrow::Cow;

use chrono::NaiveDate;
use serde_json::{ Value };
use futures::FutureExt;
use futures::future::{ ok, err };

use crate::service::configuration::Configuration;
use crate::model::{ Fahrenheit, Forecast };
use super::{ RemoteError, WeatherFuture, get_response };


pub trait RemoteAccess: Sized {    

    fn build_request_url_daily<'w>( configuration: &'w Configuration<'w>, city: &'w str, day: NaiveDate ) -> WeatherFuture< 'w, Cow<'w, str> >;

    fn build_request_url_weekly<'w>( configuration: &'w Configuration<'w>, city: &'w str ) -> WeatherFuture< 'w, Cow<'w, str> >;

    fn parse_response_daily( value: Value, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError >;

    fn parse_response_weekly( value: Value ) -> Result< Vec< Forecast<Fahrenheit> >, RemoteError >;

    fn daily_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str, day: NaiveDate ) -> WeatherFuture< 'w, Forecast<Fahrenheit> > {
        let fut = Self::build_request_url_daily( configuration, city, day )
            .then( move | url_res | {
                let fut = match url_res {
                    Ok( url ) => {
                        let fut = Self::execute_request( url ).then( move | response_res | {
                            match response_res {
                                Ok( response ) => {
                                    let fut = match Self::parse_response_daily( response, day ) {
                                        Ok( value ) => ok( value ).left_future(),
                                        Err( error ) => err( error ).right_future(),
                                    };
                                    fut.left_future()
                                },
                                Err( error ) => err( error.into() ).right_future(),
                            }
                        } );
                        fut.left_future()
                    },
                    Err( error ) => {
                        err( error ).right_future()
                    },
                };
                FutureExt::boxed( fut )
            } );
            FutureExt::boxed( fut )
    }

    fn weekly_weather<'w>( &self, configuration: &'w Configuration<'w>, city: &'w str ) -> WeatherFuture< 'w, Vec< Forecast<Fahrenheit> > > {
        let fut = Self::build_request_url_weekly( configuration, city )
            .then( | url_res | {
                let fut = match url_res {
                    Ok( url ) => {
                        let fut = Self::execute_request( url ).then( move | response_res | {
                            match response_res {
                                Ok( response ) => {
                                    let fut = match Self::parse_response_weekly( response ) {
                                        Ok( value ) => ok( value ).left_future(),
                                        Err( error ) => err( error ).right_future(),
                                    };
                                    fut.left_future()
                                },
                                Err( error ) => err( error.into() ).right_future(),
                            }
                        } );
                        fut.left_future()
                    },
                    Err( error ) => {
                        err( error ).right_future()
                    },
                };
                FutureExt::boxed( fut )
            } );
            FutureExt::boxed( fut )
    }

    fn execute_request<'w>( url: Cow<'w, str> ) -> WeatherFuture< 'w, Value > {
        // println!("execute_request :: {:?}", url);        
        get_response( url )
    }

}