

use std::borrow::Cow;
use std::ops::Add;

use crate::service::{ Configuration, CONF_PATH };
use crate::service::accuweather::{ WeahterGet };
use crate::service::remote_access::{ RemoteAccess };
use chrono::{ Local, Duration };


#[tokio::test]
#[ignore]
async fn build_request_url_daily() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.accuweather().api_key();

    let base_today = Local::today().naive_local();
    let city = "Ufa";
    let location_key = 292177;

    let today = base_today.add( Duration::days(0) );
    let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/1day/{}?apikey={}", location_key, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(1) );
    let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?apikey={}", location_key, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( |_| () ) );

    let today = base_today.add( Duration::days(2) );
    let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?apikey={}", location_key, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | _ | () ) );

    let today = base_today.add( Duration::days(6) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_biggest_day() ) );

    let today = base_today.add( Duration::days( -6) );
    assert_eq!( Err( true ), WeahterGet::build_request_url_daily( &configuration, city, today ).await.map_err( | e | e.is_smallest_day() ) );
}

#[tokio::test]
#[ignore]
async fn build_request_url_weekly() {

    let configuration = Configuration::open( CONF_PATH ).unwrap();

    let api_key = configuration.accuweather().api_key();

    let city = "Ufa";
    let location_key = 292177;

    let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?apikey={}", location_key, api_key);
    assert_eq!( Ok( Cow::Owned( url ) ), WeahterGet::build_request_url_weekly( &configuration, city ).await.map_err( |_| () ) );
}