


use std::ops::{ Add, AddAssign, Div };
use chrono::NaiveDate;
use serde::Serialize;
use super::{
    Fahrenheit,
    Celsius
};

#[derive(Debug, Clone, Serialize)]
pub struct MeasureForecast<T: Clone> {
    min: T,
    max: T,
}

impl<T: Clone> MeasureForecast<T> {
    
    pub fn new( min: T, max: T ) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn min( &self ) -> T {
        self.min.clone()
    }

    pub fn max( &self ) -> T {
        self.max.clone()
    }

}

impl<T: Clone> Default for MeasureForecast<T> where T: Default {
    
    fn default() -> MeasureForecast<T> {
        MeasureForecast {
            min: T::default(),
            max: T::default()
        }
    }

}

impl<T: Clone + Add<Output = T>> Add for MeasureForecast<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}

impl<T: Clone + Add<Output = T>> AddAssign for MeasureForecast<T> {
    
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            min: self.min.clone() + other.min,
            max: self.max.clone() + other.max,
        };
    }
}

impl<T: Clone + Div<f32, Output = T>> Div<f32> for MeasureForecast<T> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Self {
            min: self.min / rhs,
            max: self.max / rhs
        }
    }
}


#[derive(Debug, Serialize)]
pub struct Forecast<T: Clone> {
    date: NaiveDate,
    forecast: MeasureForecast<T>,
}

impl<T: Clone> Forecast<T> {
    
    pub fn new( date: NaiveDate, min: T, max: T ) -> Self {
        Self {
            date,
            forecast: MeasureForecast::new( min, max )
        }
    }

    pub fn create( date: NaiveDate, forecast: MeasureForecast<T> ) -> Self {
        Self {
            date,
            forecast
        }
    }

    pub fn date( &self ) -> &NaiveDate {
        &self.date
    }

    pub fn forecast( &self ) -> &MeasureForecast<T> {
        &self.forecast
    }

}

impl<T: Clone + From<Fahrenheit>> From<Forecast<T>> for Forecast<Option<T>> {
    
    fn from( f: Forecast<T> ) -> Forecast<Option<T>> {
        Forecast::new( f.date, f.forecast.min.into(), f.forecast.max.into() )
    }
    
}

impl<T: Clone + From<Fahrenheit>> From<&Forecast<Option<Fahrenheit>>> for Forecast<Option<T>> {
    
    fn from( f: &Forecast<Option<Fahrenheit>> ) -> Forecast<Option<T>> {
        Forecast::new( f.date, f.forecast.min.map( |f| f.into() ), f.forecast.max.map( |f| f.into() ) )
    }
    
}

impl From<Forecast<Fahrenheit>> for Forecast<Celsius> {
    
    fn from( f: Forecast<Fahrenheit> ) -> Forecast<Celsius> {
        Forecast::new( f.date, f.forecast.min.into(), f.forecast.max.into() )
    }
    
}

impl From<Forecast<Celsius>> for Forecast<Fahrenheit> {
    
    fn from( f: Forecast<Celsius> ) -> Forecast<Fahrenheit> {
        Forecast::new( f.date, f.forecast.min.into(), f.forecast.max.into() )
    }
    
}