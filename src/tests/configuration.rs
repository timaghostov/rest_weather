

use crate::service::{ Configuration, CONF_PATH };


#[test]
fn test_open_configuration_ok() {

    assert!( Configuration::open( CONF_PATH ).is_ok() );

}

#[test]
fn test_open_configuration_err() {

    assert!( Configuration::open( "configuration.toml" ).is_err() );

    assert!( Configuration::open( "xz_file" ).is_err() );

}