

use std::borrow::Cow;

use chrono::{ NaiveDate, NaiveTime, NaiveDateTime, Local };

use crate::service::remote_access::{ unix_time, get_response };


#[test]
fn test_unix_time() {
    let dt = Local::now();

    let d: NaiveDate = dt.date().naive_local();

    let unix_time = unix_time( d.clone() );

    let today = NaiveDateTime::new( d, NaiveTime::from_hms( 0, 0, 0 ) );

    assert_eq!( unix_time, today.timestamp() );

    assert_eq!( NaiveDateTime::from_timestamp( unix_time, 0 ), today );
}

#[tokio::test]
#[should_panic]
async fn test_get_response_unknown_url() {
    let _response = get_response( Cow::Borrowed( "https://www.unknown_url/" ) ).await.unwrap();
}

#[tokio::test]
async fn test_get_response_json() {
    assert!( get_response( Cow::Borrowed( "http://dataservice.accuweather.com/" ) ).await.is_ok() );
}

#[tokio::test]
#[should_panic]
async fn test_get_response_no_json() {
    let _response = get_response( Cow::Borrowed( "https://www.rust-lang.org/" ) ).await.unwrap();
}