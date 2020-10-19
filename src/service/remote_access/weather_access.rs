

#![allow(unused_imports)]


use std::error::Error;
use std::pin::Pin;
use std::borrow::Cow;
use std::time::Duration;

use chrono::NaiveDate;
use serde_json::{ Value, from_str as json_from_str };
use std::future::Future;
use futures::FutureExt;
use futures::TryFutureExt;
use futures::future::{ ok, err };
use reqwest::{ ClientBuilder, Error as ReqwestError };

use crate::service::configuration::Configuration;
use crate::model::{ Fahrenheit, Forecast };
use super::{ BadStatus, RemoteError, WeatherFuture, get_response };


pub trait RemoteAccess: Sized {    

    fn build_request_url_daily( configuration: &Configuration, city: &str, day: NaiveDate ) -> WeatherFuture< Cow<'static, str> >;

    fn build_request_url_weekly( configuration: &Configuration, city: &str ) -> WeatherFuture< Cow<'static, str> >;

    fn parse_response_daily( value: Value, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError >;

    fn parse_response_weekly( value: Value ) -> Result< Vec< Forecast<Fahrenheit> >, RemoteError >;

    fn daily_weather( &self, configuration: &Configuration<'_>, city: &str, day: NaiveDate ) -> WeatherFuture< Forecast<Fahrenheit> > {
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

    fn weekly_weather( &self, configuration: &Configuration<'_>, city: &str ) -> WeatherFuture< Vec< Forecast<Fahrenheit> > > {
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

    fn execute_request<'a>( url: Cow<'a, str> ) -> WeatherFuture< Value > {
        // println!("execute_request :: {:?}", url);        
        get_response( url.as_ref() )
    }

}