


use std::ops::Add;
use chrono::{ NaiveDate, Local, Duration };
use crate::model::{ Forecast, Fahrenheit, MeasureForecast };
use super::remote_access::{ RemoteError, MeasureError };



pub fn average_daily( day: NaiveDate, forecasts: Vec< Result< Forecast<Fahrenheit>, RemoteError > > ) -> Result< Forecast<Fahrenheit>, MeasureError  > {
    let mut count = 0;
    let mut errors = vec![];
    let mut success_measurements = MeasureForecast::default();

    for ( i, forecast_res ) in forecasts.into_iter().enumerate() {
        match forecast_res {
            Ok( forecast ) => {
                if day == *forecast.date() {
                    count += 1;
                    success_measurements += forecast.forecast().clone();
                }
            },
            Err( error ) => {
                errors.push( error );
            },
        }
    }
    
    if count > 0 {
        Ok( Forecast::create( day, success_measurements / count as f32 ) )
    } else {
        if errors.is_empty() {
            Err( MeasureError::UnknownResult )
        } else {
            Err( MeasureError::RemoteError( errors ) )
        }
    }
}

pub fn average_weekly( weekly_forecasts: Vec< Result< Vec< Forecast<Fahrenheit> >, RemoteError > > ) -> Result< Vec< Forecast<Option<Fahrenheit>> >, MeasureError > {
    let today = Local::today().naive_local();
    let mut errors = vec![];
    let mut measurements = vec![
        ( today.add( Duration::days( 0 ) ), 0, MeasureForecast::default() ),
        ( today.add( Duration::days( 1 ) ), 0, MeasureForecast::default() ),
        ( today.add( Duration::days( 2 ) ), 0, MeasureForecast::default() ),
        ( today.add( Duration::days( 3 ) ), 0, MeasureForecast::default() ),
        ( today.add( Duration::days( 4 ) ), 0, MeasureForecast::default() ),
    ];

    for ( j, weekly_forecast_res ) in weekly_forecasts.into_iter().enumerate() {
        match weekly_forecast_res {
            Ok( weekly_forecast ) => {
                for ( i, forecast ) in weekly_forecast.iter().enumerate() {
                    if measurements[i].0 == *forecast.date() {
                        measurements[i].1 += 1;
                        measurements[i].2 += forecast.forecast().clone();
                    }
                }
            },
            Err( error ) => {
                errors.push( error );
            },
        }
    }

    let mut has_measurements = false;
    let mut forecasts: Vec< Forecast::<Option<Fahrenheit>> > = vec![];
    for measurement in measurements {
        let date = measurement.0;
        let count = measurement.1;
        let measure = measurement.2;
        if count > 0 {
            has_measurements = true;
            let measure = measure / count as f32;
            let measure = MeasureForecast::new( Some( measure.min() ), Some( measure.max() ) );
            forecasts.push( Forecast::create( date, measure ) );
        } else {
            forecasts.push( Forecast::new( date, Option::<Fahrenheit>::None, Option::<Fahrenheit>::None ) );            
        }
    }
    
    if has_measurements {
        Ok( forecasts )
    } else {
        if errors.is_empty() {
            Err( MeasureError::UnknownResult )
        } else {
            Err( MeasureError::RemoteError( errors ) )
        }
    }
}