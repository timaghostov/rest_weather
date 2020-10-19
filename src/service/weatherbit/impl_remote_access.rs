
#![allow(unused_imports, unused_variables)]


use std::error::Error;
use std::borrow::Cow;

use serde_json::Value;
use chrono::NaiveDate;

use crate::model::{ Fahrenheit, Forecast };
use crate::service::configuration::Configuration;
use crate::service::remote_access::{
    RemoteAccess,
    RemoteError,
    WeatherFuture
};
use super::{ UrlBuilder, Parser };



#[derive(Debug)]
pub struct WeahterGet;

impl RemoteAccess for WeahterGet {
    
    fn build_request_url_daily( configuration: &Configuration, city: &str, day: NaiveDate ) -> WeatherFuture< Cow<'static, str> > {
        UrlBuilder::new( configuration ).city( city ).daily( day )
    }

    fn build_request_url_weekly( configuration: &Configuration, city: &str ) -> WeatherFuture< Cow<'static, str> > {
        UrlBuilder::new( configuration ).city( city ).weekly()
    }

    fn parse_response_daily( value: Value, target_day: NaiveDate ) -> Result< Forecast<Fahrenheit>, RemoteError > {
        Parser::parse_daily( value, target_day )
    }

    fn parse_response_weekly( value: Value ) -> Result< Vec< Forecast<Fahrenheit> >, RemoteError > {
        Parser::parse_weekly( value )
    }

}