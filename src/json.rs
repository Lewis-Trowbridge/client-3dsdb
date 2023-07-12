//! Accesses <https://github.com/hax0kartik/3dsdb> to get 3DS title data.
//!
//! This module uses data from a set of JSON files published by hax0kartik. A quirk of this dataset
//! is that these are divided by region. These can be accessed individually using the [Region] enum
//! with [get_releases] and [get_releases_async], or alternatively can all be accessed using
//! [get_all_releases], which is the recommended approach.
//!
//! ```
//! use client_3dsdb::json::get_all_releases;
//!
//! async fn print_releases() {
//!     let releases = get_all_releases().await;
//!
//!     for release in releases {
//!         println!("{}", release.name);
//!     }
//! }

//! ```

use futures::future::join_all;
use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

/// A title region. Required to access region-specific title lists.
#[derive(Display, Debug, EnumIter)]
pub enum Region {
    GB,
    JP,
    KR,
    TW,
    US
}

/// A 3DS title.
#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct Release {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "UID")]
    pub uid: String,
    #[serde(alias = "TitleID")]
    pub title_id: String,
    #[serde(alias = "Version")]
    pub version: String,
    #[serde(alias = "Size")]
    pub size: String,
    #[serde(alias = "Product Code")]
    pub product_code: String,
    #[serde(alias = "Publisher")]
    pub publisher: String
}

/// Gets [Release]s asynchronously for all regions
pub async fn get_all_releases() -> Vec<Release> {
    let release_futures = Region::iter().map(|region| get_releases_async(region));
    let releases = join_all(release_futures).await;
    releases.into_iter().flatten().collect()
}

/// Gets [Release]s asynchronously for a given region.
pub async fn get_releases_async(region: Region) -> Vec<Release> {
    let request = reqwest::get(&format!("https://raw.githubusercontent.com/hax0kartik/3dsdb/master/jsons/list_{}.json", region)).await.unwrap();
    request.json().await.unwrap()
}

/// Gets [Release]s synchronously for a given region.
pub fn get_releases(region: Region) -> Vec<Release> {
    let request = reqwest::blocking::get(&format!("https://raw.githubusercontent.com/hax0kartik/3dsdb/master/jsons/list_{}.json", region)).unwrap();
    request.json().unwrap()
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use once_cell::sync::Lazy;
    use rstest::*;
    use crate::json::{get_all_releases, get_releases, get_releases_async, Region, Release};

    #[rstest]
    #[case(Region::GB, "GB")]
    #[case(Region::JP, "JP")]
    #[case(Region::KR, "KR")]
    #[case(Region::TW, "TW")]
    #[case(Region::US, "US")]
    fn region_to_string_outputs_correct_string(#[case] region: Region, #[case] expected: String) {
        let actual = format!("{}", region);
        assert_eq!(actual, expected)
    }

    static EXPECTED_RELEASE: Lazy<Release> = Lazy::new(|| Release {
        name: "Shovel Software Insurance Claim".to_string(),
        uid: "50010000049535".to_string(),
        title_id: "000400000F715C00".to_string(),
        version: "N/A".to_string(),
        size: "25.7 MB [206 blocks]".to_string(),
        product_code: "KTR-N-CF6P".to_string(),
        publisher: "Batafurai".to_string()
    });

    #[rstest]
    async fn get_all_releases_returns_valid_information() {
        let releases = get_all_releases().await;
        let actual = releases.get(0).unwrap();
        assert_eq!(actual, EXPECTED_RELEASE.deref())
    }

    #[rstest]
    fn get_releases_returns_valid_information() {
        let releases = get_releases(Region::GB);
        let actual = releases.get(0).unwrap();
        assert_eq!(actual, EXPECTED_RELEASE.deref())
    }

    #[rstest]
    async fn get_releases_async_returns_valid_information() {
        let releases = get_releases_async(Region::GB).await;
        let actual = releases.get(0).unwrap();
        assert_eq!(actual, EXPECTED_RELEASE.deref())
    }
}