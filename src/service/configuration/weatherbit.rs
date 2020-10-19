

#![allow(dead_code)]


use serde::Deserialize;
use std::borrow::Cow;



#[derive( Deserialize, Debug, Clone )]
pub struct WeatherBit<'a> {
    api_key: Cow<'a, str>,
}

impl<'a> WeatherBit<'a> {
    
    pub fn api_key( &self ) -> Cow<'a, str> {
        self.api_key.to_owned()
    }

}