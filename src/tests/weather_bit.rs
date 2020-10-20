

use std::borrow::Cow;
use std::ops::Add;

use crate::service::{ Configuration, CONF_PATH };
use crate::service::weatherbit::{ WeahterGet };
use crate::service::remote_access::{ RemoteAccess };
use chrono::{ Local, Duration };


#[tokio::test]
async fn build_request_url_daily() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.weatherbit().api_key();

    let base_today = Local::today().naive_local();
    let city = "Ufa";

    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=1&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, base_today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(0) );
    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=1&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(1) );
    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=2&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(2) );
    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=3&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | _ | () ) );

    let today = base_today.add( Duration::days(15) );
    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=16&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(16) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_biggest_day() ) );

    let today = base_today.add( Duration::days( -16) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_smallest_day() ) );
}

#[tokio::test]
async fn build_request_url_weekly() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.weatherbit().api_key();

    let city = "Ufa";

    let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?units=I&city={}&days=5&key={}", city, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_weekly( &configuration, city ).await.map_err( |_| () ) );
}