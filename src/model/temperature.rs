

#![allow(dead_code)]

use serde::{ Serialize, Serializer };


use std::ops::{ Add, Div };

fn round_serialize<S>(f: &f32, s: S) -> Result<S::Ok, S::Error> where S: Serializer, {
    let scale = 1;
    let multiplier = 10f32.powi(scale as i32) as f32;
	let f = (f * multiplier).ceil() / multiplier;
    s.serialize_f32( f )
}

#[derive( Debug, Clone, Copy, Serialize )]
pub struct Fahrenheit( #[serde(serialize_with = "round_serialize")] f32);

impl Fahrenheit {
    
    pub fn new( value: f32 ) -> Self {
        Self( value )
    }

}

impl Default for Fahrenheit {
    
    fn default() -> Fahrenheit {
        Fahrenheit(0.0)
    }

}

impl From<f32> for Fahrenheit {
    
    fn from( value: f32 ) ->  Fahrenheit {
        Fahrenheit( value )
    }

}

impl Add for Fahrenheit {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self( self.0 + other.0 )
    }
}

impl Div<f32> for Fahrenheit {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Fahrenheit( self.0 / rhs )
    }
}

#[derive( Debug, Clone, Copy, Serialize )]
pub struct Celsius( #[serde(serialize_with = "round_serialize")] f32);

impl Celsius {
    
    pub fn new( value: f32 ) -> Self {
        Self( value )
    }

}

impl Default for Celsius {
    
    fn default() -> Celsius {
        Celsius(0.0)
    }

}

impl Add for Celsius {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self( self.0 + other.0 )
    }
}

impl From<f32> for Celsius {
    
    fn from( value: f32 ) ->  Celsius {
        Celsius( value )
    }

}

impl Div<f32> for Celsius {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Celsius( self.0 / rhs )
    }
}

impl From<Fahrenheit> for Celsius {
    
    fn from( fahrenheit: Fahrenheit ) ->  Celsius {
        // T(°C) = (T(°F) - 32) × 5/9
        Celsius( ( fahrenheit.0 - 32.0 ) * 5.0 / 9.0 )
    }

}

impl From<Celsius> for Fahrenheit {
    
    fn from( celsius: Celsius ) ->  Fahrenheit {
        // T(°F) = T(°C) × 9/5 + 32
        Fahrenheit( celsius.0 * 9.0 / 5.0 + 32.0 )
    }

}