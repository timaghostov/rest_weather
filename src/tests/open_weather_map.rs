

use std::borrow::Cow;
use std::ops::Add;

use crate::service::{ Configuration, CONF_PATH };
use crate::service::openweathermap::{ WeahterGet };
use crate::service::remote_access::{ RemoteAccess, unix_time };
use chrono::{ Local, Duration };


#[tokio::test]
async fn build_request_url_daily() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.openweathermap().api_key();

    let base_today = Local::today().naive_local();
    let city = "Ufa";

    let today = base_today.add( Duration::days(0) );
    let todayf = unix_time( today );
    let url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial&q={}&date={}&cnt=1&appid={}", city, todayf, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(1) );
    let todayf = unix_time( today );
    let url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial&q={}&date={}&cnt=1&appid={}", city, todayf, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(2) );
    let todayf = unix_time( today );
    let url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial&q={}&date={}&cnt=1&appid={}", city, todayf, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | _ | () ) );

    let today = base_today.add( Duration::days(15) );
    let todayf = unix_time( today );
    let url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial&q={}&date={}&cnt=1&appid={}", city, todayf, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(16) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_biggest_day() ) );

    let today = base_today.add( Duration::days( -16) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_smallest_day() ) );
}

#[tokio::test]
async fn build_request_url_weekly() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.openweathermap().api_key();

    let today = Local::today().naive_local();
    let todayf = unix_time( today );
    let city = "Ufa";

    let url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?units=imperial&q={}&date={}&cnt=5&appid={}", city, todayf, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_weekly( &configuration, city ).await.map_err( |_| () ) );
}