


use std::borrow::Cow;
use serde::Serialize;
use crate::model::{ Forecast, Fahrenheit };
use crate::service::remote_access::MeasureError;


#[derive(Debug, Serialize)]
pub struct RestResponse<'a, T: Clone + From<Fahrenheit>> {
    success: bool,
    error: Option<Cow<'a, str>>,
    forecasts: Vec<Forecast<Option<T>>>
}

impl<'a> RestResponse<'a, Fahrenheit> {
    
    pub fn cast_to<T: Clone + From<Fahrenheit>>( &self ) -> RestResponse<'a, T> {
        RestResponse {
                success: self.success,
                error: self.error.clone(),
                forecasts: self.forecasts.iter().map( | f | f.into() ).collect(),
            }
    }

    pub fn is_success( &self ) -> bool {
        self.success
    }

}

impl<'a, T: Clone + From<Fahrenheit>> From<Result< Forecast<T>, MeasureError >> for RestResponse<'a, T> {
    
    fn from( res: Result< Forecast<T>, MeasureError > ) -> RestResponse<'a, T> {
        match res {
            Ok( forecast ) => {
                RestResponse {
                    success: true,
                    error: None,
                    forecasts: vec![ forecast.into() ],
                }
            },
            Err( error ) => {
                RestResponse {
                    success: false,
                    error: Some( Cow::Owned( error.to_string() ) ),
                    forecasts: Vec::with_capacity(0),
                }
            },
        }
    }

}

impl<'a, T: Clone + From<Fahrenheit>> From<Result< Vec< Forecast<Option<T>> >, MeasureError >> for RestResponse<'a, T> {
    
    fn from( res: Result< Vec< Forecast<Option<T>> >, MeasureError > ) -> RestResponse<'a, T> {
        match res {
            Ok( forecasts ) => {
                RestResponse {
                    success: true,
                    error: None,
                    forecasts: forecasts.into(),
                }
            },
            Err( error ) => {
                RestResponse {
                    success: false,
                    error: Some( Cow::Owned( error.to_string() ) ),
                    forecasts: Vec::with_capacity(0),
                }
            },
        }
    }

}

impl<'a, T: Clone + From<Fahrenheit>> From< MeasureError > for RestResponse<'a, T> {
    fn from( error: MeasureError ) -> RestResponse<'a, T> {
        RestResponse {
            success: false,
            error: Some( Cow::Owned( error.to_string() ) ),
            forecasts: Vec::with_capacity(0),
        }
    }
}