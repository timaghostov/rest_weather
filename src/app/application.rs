


use std::ops::Add;

use chrono::{ Local, Duration };

use crate::service::Configuration;
use crate::service::Service;
use crate::service::remote_access::{ GetterWeather, RemoteError };
use crate::service::openweathermap::{ OpenWeatherMap, WeahterGet as WeahterGetOWM };
use crate::service::accuweather::{ AccuWeather, WeahterGet as WeahterGetAW };
use crate::service::weatherbit::{ WeatherBit, WeahterGet as WeahterGetWB };
use crate::model::{ Fahrenheit, Celsius, Forecast };


#[derive(Debug)]
pub struct App;

impl App {
    
    pub async fn run() {
        let configuration = Configuration::open("config/configuration.toml").unwrap();
        println!("configuration :: {:#?}", configuration);

        let mut service = Service::new( &configuration );

        service.add_remote_service( Box::new( OpenWeatherMap::new( WeahterGetOWM ) ) );
        // service.add_remote_service( Box::new( AccuWeather::new( WeahterGetAW ) ) );
        service.add_remote_service( Box::new( WeatherBit::new( WeahterGetWB ) ) );

        let today = Local::today().naive_local();
        let target_day = today.add( Duration::days(0) );
        let forecast_daily = service.forecast_daily::<Fahrenheit>( "Ufa", target_day ).await;
        println!("app run forecast_daily Fahrenheit :: {:?}", forecast_daily);
        
        let forecast_daily = service.forecast_daily::<Celsius>( "Ufa", target_day ).await;
        println!("app run forecast_daily Celsius :: {:?}", forecast_daily);
        
        let forecast_weekly = service.forecast_weekly::<Fahrenheit>( "Ufa" ).await;
        println!("app run forecast_weekly Fahrenheit :: {:?}", forecast_weekly);

        let forecast_weekly = service.forecast_weekly::<Celsius>( "Ufa" ).await;
        println!("app run forecast_weekly Celsius :: {:?}", forecast_weekly);

    }

}