use serde::Deserialize;

#[derive(Deserialize)]
struct Releases {
    #[serde(rename = "$value")]
    releases: Vec<Release>
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct Release {
    id: String,
    name: String,
    publisher: String,
    region: String,
    languages: String,
    group: String,
    #[serde(alias = "imagesize")]
    image_size: u64,
    serial: String,
    #[serde(alias = "titleid")]
    title_id: String,
    #[serde(alias = "imgcrc")]
    img_crc: String,
    filename: String,
    #[serde(alias = "releasename")]
    release_name: String,
    #[serde(alias = "trimmedsize")]
    trimmed_size: u64,
    firmware: String,
    #[serde(alias = "type")]
    _type: String,
    card: String,
}

pub fn get_releases() -> Vec<Release> {
    let request = ureq::get("http://3dsdb.com/xml.php").call().unwrap();
    let release: Releases = serde_xml_rs::from_reader(request.into_reader()).unwrap();
    release.releases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}