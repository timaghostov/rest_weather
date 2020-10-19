

#![allow(dead_code)]

use serde::Deserialize;
use std::borrow::Cow;



#[derive( Deserialize, Debug, Clone )]
pub struct AccuWeather<'a> {
    api_key: Cow<'a, str>,
}

impl<'a> AccuWeather<'a> {
    
    pub fn api_key( &self ) -> Cow<'a, str> {
        self.api_key.to_owned()
    }

}