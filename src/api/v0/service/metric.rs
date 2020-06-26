use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata;

#[straitjacket]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Metric {
    id: u64,
    name: String,
    system_name: String,
    friendly_name: String,
    description: String,
    unit: String,
    // Note, 3scale versions up to 2.8 (at least) don't include this crucial piece of data from JSON queries,
    // so there's no way to know if this has no parents or it does unless using XML, which includes it.
    // Note 2: 3scale 2.9+ uses parent_id in JSON output, and metric_id in XML output.
    metric_id: Option<u64>,
    parent_id: Option<u64>,
}

impl Metric {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn system_name(&self) -> &str {
        self.system_name.as_str()
    }

    pub fn friendly_name(&self) -> &str {
        self.friendly_name.as_str()
    }

    pub fn unit(&self) -> &str {
        self.unit.as_str()
    }

    pub fn parent_id(&self) -> Option<u64> {
        self.parent_id.or(self.metric_id)
    }
}

endpoint! { LIST, GET joining [ "/admin/api/services/", "/metrics.json"] returning Metrics }

#[cfg(test)]
mod test {
    use super::*;

    endpoint_test! { it_parses, LIST, r#"{
      "metrics": [
        {
          "metric": {
            "id": 2555418191876,
            "name": "hits",
            "system_name": "hits",
            "friendly_name": "Hits",
            "description": "Number of API hits",
            "unit": "hit",
            "created_at": "2019-03-19T09:01:01+00:00",
            "updated_at": "2019-03-19T09:01:01+00:00",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics/2555418191876"
              }
            ]
          }
        },
        {
          "metric": {
            "id": 2555418191879,
            "name": "get_slash",
            "system_name": "get_slash",
            "friendly_name": "get-slash",
            "description": "GET /",
            "unit": "hit",
            "created_at": "2019-03-19T09:04:31+00:00",
            "updated_at": "2019-03-19T09:04:31+00:00",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics/2555418191879"
              }
            ]
          }
        },
        {
          "metric": {
            "id": 2555418191880,
            "name": "post_slash",
            "system_name": "post_slash",
            "friendly_name": "post-slash",
            "description": "POST /",
            "unit": "hit",
            "created_at": "2019-03-19T09:04:33+00:00",
            "updated_at": "2019-03-19T09:04:33+00:00",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics/2555418191880"
              }
            ]
          }
        },
        {
          "metric": {
            "id": 2555418486124,
            "name": "metamethod",
            "system_name": "metamethod",
            "friendly_name": "metamethod",
            "description": "",
            "unit": "hit",
            "parent_id": 2555418191876,
            "created_at": "2020-04-05T22:43:26+01:00",
            "updated_at": "2020-04-05T22:43:26+01:00",
            "links": [
              {
                "rel": "service",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820"
              },
              {
                "rel": "self",
                "href": "https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics/2555418486124"
              }
            ]
          }
        }
      ]
    }"# }
}
