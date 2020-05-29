use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Link {
    rel: String,
    #[serde(deserialize_with = "json_extension")]
    href: String,
}

// Porta's output lacks ".json" in the href endpoints, so try to correct for that
fn json_extension<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut ss: String = serde::Deserialize::deserialize(deserializer)?;

    if !ss.ends_with(".json") {
        ss.push_str(".json")
    }

    Ok(ss)
}

impl Link {
    pub fn rel(&self) -> &str {
        self.rel.as_str()
    }

    pub fn href(&self) -> &str {
        self.href.as_str()
    }

    pub fn url(&self) -> Result<crate::deps::url::Url, Box<dyn std::error::Error>> {
        let url = crate::deps::url::Url::parse(self.href())?;
        Ok(url)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Metadata {
    created_at: String,
    updated_at: String,
    links: Option<Vec<Link>>,
}

impl Metadata {
    pub fn created_at(&self) -> &str {
        self.created_at.as_str()
    }

    pub fn updated_at(&self) -> &str {
        self.updated_at.as_str()
    }

    pub fn links(&self) -> Option<&[Link]> {
        self.links.as_ref().map(std::vec::Vec::as_slice)
    }

    pub fn find_link(&self, rel: &str) -> Option<&Link> {
        self.links()?.iter().find(|link| link.rel() == rel)?.into()
    }

    pub fn find_url(&self, rel: &str) -> Result<crate::deps::url::Url, Box<dyn std::error::Error>> {
        self.find_link(rel)
            .ok_or(format!("no {} link present", rel))?
            .url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod links {
        use super::*;

        const FIXTURE: &'static str = r#"
            [
                {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508"
                },
                {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/metrics/2555418218054.json"
                }
            ]
            "#;

        fn parse_vec(s: &str) -> Result<Vec<Link>, serde_json::Error> {
            serde_json::from_str::<Vec<Link>>(s)
        }

        #[test]
        fn it_deserializes() {
            let links = parse_vec(FIXTURE);
            assert!(links.is_ok());
        }

        #[test]
        fn it_deserializes_as_a_correctly_sized_vec() {
            let links = parse_vec(FIXTURE).expect("can't parse properly");
            assert_eq!(links.len(), 2);
        }

        #[test]
        fn it_deserializes_adding_json_extension_to_hrefs() {
            let links = parse_vec(FIXTURE).expect("can't parse properly");
            assert!(links.iter().all(|l| l.href().ends_with(".json")));
        }

        #[test]
        fn it_deserializes_avoiding_adding_json_extension_when_already_existing_to_hrefs() {
            let links = parse_vec(FIXTURE).expect("can't parse properly");
            // NOTE: the second entry in the fixture already ends with ".json"
            assert!(links.iter().all(|l| !l.href().ends_with(".json.json")));
        }
    }

    mod metadata {
        use super::*;

        const FIXTURE: &'static str = r#"{
            "created_at": "2020-05-11T13:55:00+01:00",
            "updated_at": "2020-05-11T13:55:00+01:00",
            "links": [
                {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508"
                },
                {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417783508/metrics/2555418218054.json"
                }
            ]
        }"#;

        fn parse_metadata(s: &str) -> Result<Metadata, serde_json::Error> {
            serde_json::from_str::<Metadata>(s)
        }

        #[test]
        fn it_parses_metadata() {
            let metadata = parse_metadata(FIXTURE);
            match &metadata {
                Err(e) => println!("Error parsing metadata {:#?}", e),
                _ => (),
            }
            assert!(metadata.is_ok());
        }

        #[test]
        fn it_finds_the_service_link() {
            let metadata = parse_metadata(FIXTURE).expect("can't parse properly");
            assert!(metadata.find_link("service").is_some());
        }

        #[test]
        fn it_finds_the_self_link() {
            let metadata = parse_metadata(FIXTURE).expect("can't parse properly");
            assert!(metadata.find_link("self").is_some());
        }

        #[test]
        fn it_fails_to_find_a_non_existing_link() {
            let metadata = parse_metadata(FIXTURE).expect("can't parse properly");
            assert!(metadata.find_link("non_existing").is_none());
        }

        #[test]
        fn it_converts_a_link_to_a_url() {
            let metadata = parse_metadata(FIXTURE).expect("can't parse properly");
            let url = metadata.find_url("service");
            match &url {
                Err(e) => println!("Error converting a link to a URL {:#?}", e),
                _ => (),
            }
            assert!(url.is_ok());
        }
    }
}
