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
    use crate::json::Region;

    #[test]
    fn region_to_string_outputs_correct_string() {
        let expected = "GB".to_string();
        let actual_enum = Region::GB;
        let actual = actual_enum.to_string();
        assert_eq!(actual, expected)
    }
}