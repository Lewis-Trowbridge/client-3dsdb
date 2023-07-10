//! A client library for 3DS title DBs.
//!
//! This library can use multiple sources to get title data, each with pros and cons. Each source has its own module with associated structs and methods. So far there are:
//!
//! - xml: http://3dsdb.com
//! - json: https://github.com/hax0kartik/3dsdb

#[cfg(feature = "xml")]
pub mod xml;
#[cfg(feature = "json")]
mod json;


