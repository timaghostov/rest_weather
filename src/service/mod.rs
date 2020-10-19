

mod configuration;
pub mod accuweather;
pub mod openweathermap;
pub mod weatherbit;
pub mod remote_access;

mod webservice;
mod util;


pub use configuration::Configuration;
pub use webservice::Service;