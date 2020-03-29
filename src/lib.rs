use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Link {
    rel: String,
    href: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    id: u64,
    metric_id: u64,
    pattern: String,
    http_method: String,
    delta: u64,
    position: u64,
    last: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Metadata {
    created_at: String,
    updated_at: String,
    links: Vec<Link>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MappingRuleAndMetadata {
    #[serde(flatten)]
    item: MappingRule,
    #[serde(flatten, skip_serializing)]
    metadata: Option<Metadata>,
    //#[serde(flatten, skip_serializing)]
    //// add a serde catch all?
    //extra: HashMap<_key??, value??>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MappingRuleTag {
    #[serde(rename = "mapping_rule")]
    MappingRules(MappingRuleAndMetadata),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MappingRules {
    mapping_rules: Vec<MappingRuleTag>,
}

impl From<Vec<MappingRule>> for MappingRules {
    fn from(mrvec: Vec<MappingRule>) -> Self {
        MappingRules {
            mapping_rules: mrvec
                .into_iter()
                .map(|mapping_rule| MappingRuleTag::MappingRules( MappingRuleAndMetadata {
                    item: mapping_rule,
                    metadata: None,
                })).collect::<Vec<_>>(),
        }
    }
}

impl From<MappingRules> for Vec<MappingRuleAndMetadata> {
    fn from(mr: MappingRules) -> Self {
        mr.mapping_rules.into_iter().map(|mr| {
            let MappingRuleTag::MappingRules(mramd) = mr;
            mramd
        }).collect()
    }
}

impl From<MappingRules> for Vec<MappingRule> {
    fn from(mr: MappingRules) -> Self {
        mr.mapping_rules.into_iter().map(|mr| {
            let MappingRuleTag::MappingRules(mramd) = mr;
            mramd.item
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_serializes() {
        let mapping_rules = MappingRules::from(vec![
            MappingRule {
                id: 375841,
                metric_id: 2555418191879,
                pattern: "/".into(),
                http_method: "GET".into(),
                delta: 1,
                position: 1,
                last: false,
                //metadata: Some(Metadata {
                //    created_at: "2019-03-19T09:04:35Z".into(),
                //    updated_at: "2019-03-19T09:04:39Z".into(),
                //    links: vec![
                //        Link {
                //            rel: "self".into(),
                //            href: "/admin/api/services/2555417777820/proxy/mapping_rules/375841".into(),
                //        },
                //        Link {
                //            rel: "service".into(),
                //            href: "/admin/api/services/2555417777820".into(),
                //        },
                //        Link {
                //            rel: "proxy".into(),
                //            href: "/admin/api/services/2555417777820/proxy".into(),
                //        },
                //    ],
                //}),
            },
            MappingRule {
                id: 375842,
                metric_id: 2555418191880,
                pattern: "/".into(),
                http_method: "POST".into(),
                delta: 1,
                position: 2,
                last: false,
                //metadata: Some(Metadata {
                //    created_at: "2019-03-19T09:04:36Z".into(),
                //    updated_at: "2019-03-19T09:04:39Z".into(),
                //    links: vec![
                //        Link {
                //            rel: "self".into(),
                //            href: "/admin/api/services/2555417777820/proxy/mapping_rules/375842".into(),
                //        },
                //        Link {
                //            rel: "service".into(),
                //            href: "/admin/api/services/2555417777820".into(),
                //        },
                //        Link {
                //            rel: "proxy".into(),
                //            href: "/admin/api/services/2555417777820/proxy".into(),
                //        }
                //    ],
                //}),
            }
        ]);
        println!("{}", serde_json::to_string_pretty(&mapping_rules).unwrap());
    }

    #[test]
    fn it_parses() {
        let body = r##"{
  "mapping_rules": [
    {
      "mapping_rule": {
        "id": 375841,
        "metric_id": 2555418191879,
        "pattern": "/",
        "http_method": "GET",
        "delta": 1,
        "position": 1,
        "last": false,
        "created_at": "2019-03-19T09:04:35Z",
        "updated_at": "2019-03-19T09:04:39Z",
        "links": [
          {
            "rel": "self",
            "href": "/admin/api/services/2555417777820/proxy/mapping_rules/375841"
          },
          {
            "rel": "service",
            "href": "/admin/api/services/2555417777820"
          },
          {
            "rel": "proxy",
            "href": "/admin/api/services/2555417777820/proxy"
          }
        ]
      }
    },
    {
      "mapping_rule": {
        "id": 375842,
        "metric_id": 2555418191880,
        "pattern": "/",
        "http_method": "POST",
        "delta": 1,
        "position": 2,
        "last": false,
        "created_at": "2019-03-19T09:04:36Z",
        "updated_at": "2019-03-19T09:04:39Z",
        "links": [
          {
            "rel": "self",
            "href": "/admin/api/services/2555417777820/proxy/mapping_rules/375842"
          },
          {
            "rel": "service",
            "href": "/admin/api/services/2555417777820"
          },
          {
            "rel": "proxy",
            "href": "/admin/api/services/2555417777820/proxy"
          }
        ]
      }
    }
  ]
}"##;
        let mapping_rules: Result<MappingRules, _> = serde_json::from_str(&body);
        assert!(mapping_rules.is_ok());
    }
}
