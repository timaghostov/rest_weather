

use std::ops::Add;

use chrono::{ Local, Duration };

use crate::service::{ Configuration, CONF_PATH };
use crate::service::Service;
use crate::service::openweathermap::{ OpenWeatherMap, WeahterGet as WeahterGetOWM };
use crate::service::accuweather::{ AccuWeather, WeahterGet as WeahterGetAW };
use crate::service::weatherbit::{ WeatherBit, WeahterGet as WeahterGetWB };
use crate::model::{ Fahrenheit };


#[tokio::test]
#[ignore]
async fn test() {
    let configuration = Configuration::open(CONF_PATH).unwrap();
    
    let mut service = Service::new( configuration );

    service.add_remote_service( Box::new( OpenWeatherMap::new( WeahterGetOWM ) ) );
    service.add_remote_service( Box::new( AccuWeather::new( WeahterGetAW ) ) );
    service.add_remote_service( Box::new( WeatherBit::new( WeahterGetWB ) ) );

    let today = Local::today().naive_local();
    let city = "Ufa";

    let target_day = today.add( Duration::days(0) );
    let _forecast_daily = service.forecast_daily::<Fahrenheit>( city, target_day ).await;

    assert!( _forecast_daily.is_success() );

    let target_day = today.add( Duration::days(10) );
    let _forecast_daily = service.forecast_daily::<Fahrenheit>( city, target_day ).await;

    assert!( _forecast_daily.is_success() );

    let target_day = today.add( Duration::days(20) );
    let _forecast_daily = service.forecast_daily::<Fahrenheit>( city, target_day ).await;
    
    assert_eq!( _forecast_daily.is_success(), false );

    let target_day = today.add( Duration::days(-20) );
    let _forecast_daily = service.forecast_daily::<Fahrenheit>( city, target_day ).await;

    assert_eq!( _forecast_daily.is_success(), false );
    
    let _forecast_weekly = service.forecast_weekly::<Fahrenheit>( city ).await;
    
    assert!( _forecast_weekly.is_success() );
}