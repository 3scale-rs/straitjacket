use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Eternity,
}

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    id: u64,
    metric_id: u64,
    plan_id: u64,
    period: Period,
    value: u64,
}

endpoint! { EP_LIST_LIMITS, GET joining [ "/admin/api/application_plans/", "/limits.json"] returning Limits }
endpoint_test! { it_parses, EP_LIST_LIMITS, r##"{
   "limits":[
      {
         "limit":{
            "id":2639696107074,
            "period":"day",
            "value":5,
            "metric_id":2555418191879,
            "plan_id":2357356012630,
            "created_at":"2019-03-19T09:04:54+00:00",
            "updated_at":"2019-03-19T09:04:54+00:00",
            "links":[
               {
                  "rel":"metric",
                  "href":"https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/metrics/2555418191879"
               },
               {
                  "rel":"self",
                  "href":"https://istiodevel-admin.3scale.net/admin/api/application_plans/2357356012630/metrics/2555418191879/limits?id=2639696107074"
               },
               {
                  "rel":"plan",
                  "href":"https://istiodevel-admin.3scale.net/admin/api/services/2555417777820/application_plans/2357356012630"
               }
            ]
         }
      }
]
}"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let limits = Limits::from(vec![Limit {
            id: 2639696107074,
            metric_id: 2555418191879,
            plan_id: 2357356012630,
            period: Period::Day,
            value: 5,
        }]);
        let result = serde_json::to_string_pretty(&limits);
        match result {
            Err(ref e) => println!("Error: {:#?}", e),
            _ => (),
        }
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
