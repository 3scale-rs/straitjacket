use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    id: u64,
    name: String,
    system_name: Option<String>,
    account_id: Option<String>,
    scope: String,
    visible: bool,
    description: Option<String>,
}

endpoint! { EP_LIST_FEATURES, GET joining [ "/admin/api/features.json"] returning Features }
endpoint_test! { it_parses, EP_LIST_FEATURES, r##"{
   "features":[
      {
         "feature":{
            "id":4142037,
            "name":"my_feature",
            "system_name":"my_feature",
            "scope":"account_plan",
            "visible":true,
            "created_at":"2020-05-04T19:03:52+01:00",
            "updated_at":"2020-05-04T19:03:52+01:00",
            "links":[
               {
                  "rel":"self",
                  "href":"https://istiodevel-admin.3scale.net/admin/api/features/4142037"
               }
            ]
         }
      },
      {
         "feature":{
            "id":4142038,
            "name":"my_2nd_feature",
            "system_name":"my_2nd_feature",
            "scope":"account_plan",
            "visible":true,
            "created_at":"2020-05-04T19:05:06+01:00",
            "updated_at":"2020-05-04T19:05:06+01:00",
            "links":[
               {
                  "rel":"self",
                  "href":"https://istiodevel-admin.3scale.net/admin/api/features/4142038"
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
        let features = Features::from(vec![
            Feature {
                id: 4142037,
                name: "my_feature".into(),
                system_name: Some(String::from("my_feature")),
                account_id: None,
                scope: "account_plan".into(),
                visible: true,
                description: None,
            },
            Feature {
                id: 4142038,
                name: "my_2nd_feature".into(),
                system_name: Some(String::from("my_2nd_feature")),
                visible: false,
                scope: "account_plan".into(),
                account_id: None,
                description: None,
            },
        ]);
        let result = serde_json::to_string_pretty(&features);
        match result {
            Err(ref e) => println!("Error: {:#?}", e),
            _ => (),
        }
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
