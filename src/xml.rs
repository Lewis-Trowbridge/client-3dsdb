//! Accesses <http://3dsdb.com> to get 3DS title data.
//!
//! This module uses data from an XML file published on 3dsdb.com. There are two methods to
//! access these being [get_releases] and [get_releases_async], which are equivalent bar the async
//! usage.
//!
//! ```
//! use client_3dsdb::xml::get_releases;
//!
//! fn print_releases() {
//!     let releases = get_releases();
//!
//!     for release in releases {
//!         println!("{}", release.name);
//!     }
//! }
//!
//! ```
//!
use serde::Deserialize;

#[derive(Deserialize)]
struct Releases {
    #[serde(rename = "$value")]
    releases: Vec<Release>
}

/// A 3DS title.
#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub region: String,
    pub languages: String,
    pub group: String,
    #[serde(alias = "imagesize")]
    pub image_size: u64,
    pub serial: String,
    #[serde(alias = "titleid")]
    pub title_id: String,
    #[serde(alias = "imgcrc")]
    pub img_crc: String,
    pub filename: String,
    #[serde(alias = "releasename")]
    pub release_name: String,
    #[serde(alias = "trimmedsize")]
    pub trimmed_size: u64,
    pub firmware: String,
    #[serde(alias = "type")]
    pub _type: String,
    pub card: String,
}

/// Gets of [Release]s asynchronously.
pub async fn get_releases_async() -> Vec<Release> {
    let request = reqwest::get("http://3dsdb.com/xml.php").await.unwrap();
    let release: Releases = serde_xml_rs::from_str(&request.text().await.unwrap()).unwrap();
    release.releases
}

/// Gets [Release]s synchronously.
pub fn get_releases() -> Vec<Release> {
    let request = reqwest::blocking::get("http://3dsdb.com/xml.php").unwrap();
    let release: Releases = serde_xml_rs::from_str(&request.text().unwrap()).unwrap();
    release.releases
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    fn get_releases_gets_valid_information() {
        let expected = Release {
            id: "1".to_string(),
            name: "Tom Clancys Ghost Recon: Shadow Wars".to_string(),
            publisher: "Ubisoft".to_string(),
            region: "EUR".to_string(),
            languages: "en,fr,de,it,es".to_string(),
            group: "Legacy".to_string(),
            image_size: 2048,
            serial: "CTR-AGRP".to_string(),
            title_id: "0004000000037500".to_string(),
            img_crc: "5BD0B123".to_string(),
            filename: "lgc-grsw".to_string(),
            release_name: "Tom_Clancys_Ghost_Recon_Shadow_Wars_EUR_3DS-LGC".to_string(),
            trimmed_size: 229750272,
            firmware: "1.0.0E".to_string(),
            _type: "1".to_string(),
            card: "1".to_string()
        };
        let value = get_releases();
        let actual = value.get(0).unwrap();
        assert_eq!(actual, &expected)
    }

    #[rstest]
    async fn get_releases_async_gets_valid_information() {
        let expected = Release {
            id: "1".to_string(),
            name: "Tom Clancys Ghost Recon: Shadow Wars".to_string(),
            publisher: "Ubisoft".to_string(),
            region: "EUR".to_string(),
            languages: "en,fr,de,it,es".to_string(),
            group: "Legacy".to_string(),
            image_size: 2048,
            serial: "CTR-AGRP".to_string(),
            title_id: "0004000000037500".to_string(),
            img_crc: "5BD0B123".to_string(),
            filename: "lgc-grsw".to_string(),
            release_name: "Tom_Clancys_Ghost_Recon_Shadow_Wars_EUR_3DS-LGC".to_string(),
            trimmed_size: 229750272,
            firmware: "1.0.0E".to_string(),
            _type: "1".to_string(),
            card: "1".to_string()
        };
        let value = get_releases_async().await;
        let actual = value.get(0).unwrap();
        assert_eq!(actual, &expected)
    }
}