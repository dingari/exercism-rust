extern crate chrono;

use chrono::*;

pub fn after<T: TimeZone>(start_date: DateTime<T>) -> DateTime<T> {
	start_date.checked_add(Duration::seconds(1000000000)).unwrap()
}