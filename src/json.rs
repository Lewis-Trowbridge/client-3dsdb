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
struct Release {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "UID")]
    uid: String,
    #[serde(alias = "TitleID")]
    title_id: String,
    #[serde(alias = "Version")]
    version: String,
    #[serde(alias = "Size")]
    size: String,
    #[serde(alias = "Product Code")]
    product_code: String,
    #[serde(alias = "Publisher")]
    publisher: String
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::json::Region;

    #[rstest]
    #[case(Region::GB, "GB")]
    #[case(Region::JP, "JP")]
    #[case(Region::KR, "KR")]
    #[case(Region::TW, "TW")]
    #[case(Region::US, "US")]
    fn region_to_string_outputs_correct_string(#[case] region: Region, #[case] expected: String) {
        let actual = region.to_string();
        assert_eq!(actual, expected)
    }
}