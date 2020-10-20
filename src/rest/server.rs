


use std::process::exit;
use std::io;

use actix_web::{ get, web, App, HttpServer, HttpResponse };
use chrono::NaiveDate;
use crate::model::{ Fahrenheit, Celsius };
use crate::rest::RestResponse;
use crate::service::{ Configuration, CONF_PATH };
use crate::service::Service;
use crate::service::remote_access::MeasureError;
use crate::service::openweathermap::{ OpenWeatherMap, WeahterGet as WeahterGetOWM };
use crate::service::accuweather::{ AccuWeather, WeahterGet as WeahterGetAW };
use crate::service::weatherbit::{ WeatherBit, WeahterGet as WeahterGetWB };


pub const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug)]
pub struct RestServer;

async fn forecast_daily<'a, T: Clone + From<Fahrenheit>>(service: &Service<'a>, city: &str, date: &str) -> RestResponse<'a, T> {
    let date_res = NaiveDate::parse_from_str( &date, DATE_FORMAT );
    match date_res {
        Ok( date ) => {
            service.forecast_daily::<T>( city, date ).await
        },
        Err( _ ) => MeasureError::BadDateFormat.into(),
    }
}

async fn forecast_weekly<'a, T: Clone + From<Fahrenheit>>(service: &Service<'a>, city: &str) -> RestResponse<'a, T> {
    service.forecast_weekly::<T>( city ).await
}

#[get("{city}/daily/{date}")]
async fn daily_fahrenheit(service: web::Data<Service<'_>>, web::Path(( city, date )): web::Path<(String, String)>) -> HttpResponse {
    println!("weekly :: fahrenheit :: {} :: {}", city, date );
    let response = forecast_daily::<Fahrenheit>( &service, &city, &date ).await;
    match serde_json::to_string(&response) {
        Ok( response ) => {
            HttpResponse::Ok().body(response)
        },
        Err( _error ) => {
            HttpResponse::from( "Failed on response convert." )
        },
    }
}

#[get("{city}/daily/{date}/celsius")]
async fn daily_celsius(service: web::Data<Service<'_>>, web::Path(( city, date )): web::Path<(String, String)>) -> HttpResponse {
    println!("weekly :: celsius :: {} :: {}", city, date );
    let response = forecast_daily::<Celsius>( &service, &city, &date ).await;
    match serde_json::to_string(&response) {
        Ok( response ) => {
            HttpResponse::Ok().body(response)
        },
        Err( _error ) => {
            HttpResponse::from( "Failed on response convert." )
        },
    }
}

#[get("{city}/weekly")]
async fn weekly_fahrenheit(service: web::Data<Service<'_>>, web::Path( city ): web::Path< String >) -> HttpResponse {
    println!("weekly :: fahrenheit :: {}", city);    
    let response = forecast_weekly::<Fahrenheit>( &service, &city ).await;
    match serde_json::to_string(&response) {
        Ok( response ) => {
            HttpResponse::Ok().body(response)
        },
        Err( _error ) => {
            HttpResponse::from( "Failed on response convert." )
        },
    }
}

#[get("{city}/weekly/celsius")]
async fn weekly_celsius(service: web::Data<Service<'_>>, web::Path( city ): web::Path< String >) -> HttpResponse {
    println!("weekly :: celsius :: {}", city);
    let response = forecast_weekly::<Celsius>( &service, &city ).await;
    match serde_json::to_string(&response) {
        Ok( response ) => {
            HttpResponse::Ok().body(response)
        },
        Err( _error ) => {
            HttpResponse::from( "Failed on response convert." )
        },
    }
}

impl RestServer {
    
    pub async fn run() -> io::Result<()> {
        
        let configuration = match Configuration::open(CONF_PATH) {
            Ok(configuration) => configuration,
            Err( _error ) => {
                println!("Error. Cannot read configuration file on {}", CONF_PATH);
                exit(0);
            },
        };

        let address = format!("{}:{}", configuration.host(), configuration.port());

        HttpServer::new(move || {
            let conf = configuration.clone();
            let mut service = Service::new( conf );

            service.add_remote_service( Box::new( OpenWeatherMap::new( WeahterGetOWM ) ) );
            service.add_remote_service( Box::new( AccuWeather::new( WeahterGetAW ) ) );
            service.add_remote_service( Box::new( WeatherBit::new( WeahterGetWB ) ) );

            let data = web::Data::new( service );

            App::new()
                .app_data( data )
                .service( daily_fahrenheit )
                .service( daily_celsius )
                .service( weekly_fahrenheit )
                .service( weekly_celsius )
        })
        .bind( &address )?
        .run()
        .await
        
    }

}