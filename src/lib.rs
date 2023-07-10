//! A client library for 3DS title DBs.
//!
//! This library can use multiple sources to get title data, each with pros and cons. Each source has its own module with associated structs and methods. So far there are:
//!
//! - xml: 3dsdb.com

#[cfg(feature = "xml")]
pub mod xml;


