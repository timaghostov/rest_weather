


use serde::Deserialize;
use toml::{ self, Value as TomlValue };

use std::fs;
use std::path::Path;
use std::borrow::Cow;

pub use super::openweathermap::OpenWeatherMap;
pub use super::accuweather::AccuWeather;
pub use super::weatherbit::WeatherBit;
pub use super::error::ConfigurationError;

pub const CONF_PATH: &str = "config/configuration.toml";

#[derive( Deserialize, Debug, Clone )]
pub struct Configuration<'a> {
    openweathermap: OpenWeatherMap<'a>,
    accuweather: AccuWeather<'a>,
    weatherbit: WeatherBit<'a>,
    port: u16,
    host: Cow<'a, str>,
}

impl<'a> Configuration<'a> {

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Configuration<'a>, ConfigurationError> {
        Self::parse( path )?.try_into().map_err( ConfigurationError::from )
    }
    
    fn parse<P: AsRef<Path>>(path: P) -> Result<TomlValue, ConfigurationError> {
        let content = fs::read_to_string( path )?;
        let value = content.parse::<toml::Value>()?;
        Ok( value )
    }

    pub fn port( &self ) -> u16 {
        self.port
    }

    pub fn host( &self ) -> Cow<'a, str> {
        self.host.clone()
    }

    pub fn openweathermap( &self ) -> Cow<'a, OpenWeatherMap> {
        Cow::Borrowed( &self.openweathermap )
    }

    pub fn accuweather( &self ) -> Cow<'a, AccuWeather> {
        Cow::Borrowed( &self.accuweather )
    }

    pub fn weatherbit( &self ) -> Cow<'a, WeatherBit> {
        Cow::Borrowed( &self.weatherbit )
    }

    // pub fn api_key_openweathermap( &self ) -> Cow<'a, str> {
    //     self.openweathermap.api_key()
    // }

    // pub fn api_key_accuweather( &self ) -> Cow<'a, str> {
    //     self.accuweather.api_key()
    // }

    // pub fn api_key_weatherbit( &self ) -> Cow<'a, str> {
    //     self.weatherbit.api_key()
    // }

}