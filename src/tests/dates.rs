


use std::ops::Add;

use chrono::{ Local, NaiveDate, Duration };
use crate::rest::DATE_FORMAT;


#[test]
fn test_parse_date_ok() {
    assert!( NaiveDate::parse_from_str( "2020-10-20", DATE_FORMAT ).is_ok() );
}

#[test]
fn test_parse_date_err() {
    assert!( NaiveDate::parse_from_str( "20.10.2020", DATE_FORMAT ).is_err() );
}

#[test]
fn test_parse_date_add_today() {
    let today = Local::today().naive_local();

    assert_eq!( today, today.add( Duration::days(0) ) );

    assert_eq!( today.add( Duration::days(0) ) - today, Duration::days(0) );

}

#[test]
fn test_parse_date_add_n() {
    let today = Local::today().naive_local();

    let n = 17;

    assert_ne!( today, today.add( Duration::days( n ) ) );

    assert_eq!( today.add( Duration::days(n) ) - today, Duration::days(n) );

}