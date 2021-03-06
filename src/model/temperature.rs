


use serde::Serialize;


use std::ops::{ Add, Div };


#[derive( Debug, Clone, Copy, Serialize )]
pub struct Fahrenheit(f32);

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
pub struct Celsius(f32);

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