use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;

#[derive(Debug)]
pub enum Region {
    GB,
    JP,
    KR,
    TW,
    US
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

/// Gets a vec of [Release]s from hax0kartik's repository.
pub fn get_releases(region: Region) -> Vec<Release> {
    let request = ureq::get(&format!("https://raw.githubusercontent.com/hax0kartik/3dsdb/master/jsons/list_{}.json", region)).call().unwrap();
    serde_json::from_reader(request.into_reader()).unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::json::{get_releases, Region, Release};

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

    #[test]
    fn get_releases_returns_valid_information() {
        let expected = Release{
            name: "Shovel Software Insurance Claim".to_string(),
            uid: "50010000049535".to_string(),
            title_id: "000400000F715C00".to_string(),
            version: "N/A".to_string(),
            size: "25.7 MB [206 blocks]".to_string(),
            product_code: "KTR-N-CF6P".to_string(),
            publisher: "Batafurai".to_string()
        };
        let releases = get_releases(Region::GB);
        let actual = releases.get(0).unwrap();
        assert_eq!(actual, &expected)
    }
}