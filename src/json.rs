use serde::Deserialize;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

#[derive(Display, Debug, EnumIter)]
pub enum Region {
    GB,
    JP,
    KR,
    TW,
    US
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

pub async fn get_releases_async(region: Region) -> Vec<Release> {
    let request = reqwest::get(&format!("https://raw.githubusercontent.com/hax0kartik/3dsdb/master/jsons/list_{}.json", region)).await.unwrap();
    request.json().await.unwrap()
}

/// Gets a vec of [Release]s from hax0kartik's repository.
pub fn get_releases(region: Region) -> Vec<Release> {
    let request = reqwest::blocking::get(&format!("https://raw.githubusercontent.com/hax0kartik/3dsdb/master/jsons/list_{}.json", region)).unwrap();
    request.json().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::json::{get_releases, get_releases_async, Region, Release};

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

    #[rstest]
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

    #[rstest]
    async fn get_releases_async_returns_valid_information() {
        let expected = Release{
            name: "Shovel Software Insurance Claim".to_string(),
            uid: "50010000049535".to_string(),
            title_id: "000400000F715C00".to_string(),
            version: "N/A".to_string(),
            size: "25.7 MB [206 blocks]".to_string(),
            product_code: "KTR-N-CF6P".to_string(),
            publisher: "Batafurai".to_string()
        };
        let releases = get_releases_async(Region::GB).await;
        let actual = releases.get(0).unwrap();
        assert_eq!(actual, &expected)
    }
}