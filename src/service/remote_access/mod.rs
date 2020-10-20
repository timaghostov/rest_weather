


mod weather_access;
mod getter_weather;
mod error;

pub use weather_access::RemoteAccess;
pub use error::{ BadStatus, RemoteError, MeasureError };
pub use getter_weather::GetterWeather;

use serde_json::Value;
use reqwest::{ self, Error as ReqwestError };
use chrono::{ NaiveDate, Datelike };
use futures::future::{ BoxFuture, FutureExt, ok, err };
use std::error::Error;
use std::time::Duration;
use std::borrow::Cow;


pub type WeatherFuture<'w, T> = BoxFuture< 'w, Result< T, RemoteError > >;


pub fn unix_time( date: NaiveDate ) -> i64 {
    const UNIX_EPOCH_DAY: i64 = 719_163;
    let gregorian_day = i64::from(date.num_days_from_ce());
    (gregorian_day - UNIX_EPOCH_DAY) * 86_400
}

pub fn get_response<'w>( url: Cow<'w, str> ) -> WeatherFuture<'w, Value> {
    // println!("get_response url :: {:?}", url);
    let client_res = reqwest::Client::builder()
                            // .timeout(Duration::from_secs(10))
                            .build();
    let fut = match client_res {
        Ok( client ) => {
            let fut = client.get( url.as_ref() )
                .send()
                .then( | response_res | {
                    // println!("response_res :: {:?}", response_res);
                    let fut = match response_res {
                        Ok( response ) => {
                            response.json::<Value>().then( | value_res | {
                                // println!("get_response value_res :: {:?}", value_res);
                                match value_res {
                                    Ok( value ) => ok( value ).left_future(),
                                    Err( error ) => err( RemoteError::ReqwestError(  error ) ).right_future(),
                                }
                            } ).left_future()
                        },
                        Err( error ) => err( RemoteError::ReqwestError( error ) ).right_future(),
                    };
                    fut
                } );
            fut.left_future()
        },
        Err( error ) => err( RemoteError::ReqwestError( error ) ).right_future(),
    };
    FutureExt::boxed( fut )
}