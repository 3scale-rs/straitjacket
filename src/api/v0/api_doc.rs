use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDoc {
    id: u64,
    system_name: String,
    name: String,
    published: bool,
    skip_swagger_validations: bool,
    body: String,
}

endpoint! { EP_LIST_API_DOCS, GET joining [ "/admin/api/active_docs.json"] returning ApiDocs }
endpoint_test! { it_parses, EP_LIST_API_DOCS, r##"{
   "api_docs" : [
      {
         "api_doc" : {
            "body" : "  {\n      \"swagger\": \"2.0\",\n      \"info\": {\n          \"version\": \"1.0.0\",\n          \"title\": \"Echo API\",\n          \"description\": \"A sample echo API\"\n      },\n      \"host\": \"echo-api.3scale.net\",\n      \"basePath\": \"/\",\n      \"schemes\": [\n          \"http\"\n      ],\n      \"consumes\": [\n          \"application/json\"\n      ],\n      \"produces\": [\n          \"application/json\"\n      ],\n      \"paths\": {\n          \"/\": {\n              \"get\": {\n                  \"description\": \"Echo API with no parameters\",\n                  \"operationId\": \"echo_no_params\",\n                  \"produces\": [\n                      \"application/json\",\n                      \"application/xml\",\n                      \"text/xml\",\n                      \"text/html\"\n                  ],\n                  \"parameters\": [\n                      {\n                          \"name\": \"user_key\",\n                          \"in\": \"query\",\n                          \"description\": \"Your API access key\",\n                          \"required\": true,\n                          \"x-data-threescale-name\": \"user_keys\",\n                          \"type\": \"string\"\n                      }\n                  ],\n                  \"responses\": {\n                      \"200\": {\n                          \"description\": \"response\",\n                          \"schema\": {\n                              \"$ref\": \"#/definitions/ResponseModel\"\n                          }\n                      },\n                      \"default\": {\n                          \"description\": \"unexpected error\",\n                          \"schema\": {\n                              \"$ref\": \"#/definitions/ErrorModel\"\n                          }\n                      }\n                  }\n              }\n          },\n          \"/{echo}\": {\n              \"get\": {\n                  \"description\": \"Echo API with parameters\",\n                  \"operationId\": \"echo_with_params\",\n                  \"produces\": [\n                      \"application/json\",\n                      \"application/xml\",\n                      \"text/xml\",\n                      \"text/html\"\n                  ],\n                  \"parameters\": [\n                      {\n                          \"name\": \"echo\",\n                          \"in\": \"path\",\n                          \"description\": \"The string to be echoed\",\n                          \"required\": true,\n                          \"type\": \"string\"\n                      },\n                      {\n                          \"name\": \"user_key\",\n                          \"in\": \"query\",\n                          \"description\": \"Your API access key\",\n                          \"required\": true,\n                          \"x-data-threescale-name\": \"user_keys\",\n                          \"type\": \"string\"\n                      }\n                  ],\n                  \"responses\": {\n                      \"200\": {\n                          \"description\": \"response\",\n                          \"schema\": {\n                              \"$ref\": \"#/definitions/ResponseModel\"\n                          }\n                      },\n                      \"default\": {\n                          \"description\": \"unexpected error\",\n                          \"schema\": {\n                              \"$ref\": \"#/definitions/ErrorModel\"\n                          }\n                      }\n                  }\n              }\n          }\n      },\n      \"definitions\": {\n          \"ResponseModel\": {\n              \"type\": \"object\",\n              \"required\": [\n                  \"method\",\n                  \"path\",\n                  \"args\",\n                  \"headers\"\n              ],\n              \"properties\": {\n                  \"method\": {\n                      \"type\": \"string\"\n                  },\n                  \"path\": {\n                      \"type\": \"string\"\n                  },\n                  \"args\": {\n                      \"type\": \"string\"\n                  },\n                  \"headers\": {\n                      \"type\": \"object\"\n                  }\n              }\n          },\n          \"ErrorModel\": {\n              \"type\": \"object\",\n              \"required\": [\n                  \"code\",\n                  \"message\"\n              ],\n              \"properties\": {\n                  \"code\": {\n                      \"type\": \"integer\",\n                      \"format\": \"int32\"\n                  },\n                  \"message\": {\n                      \"type\": \"string\"\n                  }\n              }\n          }\n      }\n  }\n",
            "skip_swagger_validations" : false,
            "id" : 89270,
            "published" : true,
            "system_name" : "echo",
            "created_at" : "2018-07-06T12:30:23+01:00",
            "name" : "Echo",
            "updated_at" : "2018-07-06T12:30:23+01:00"
         }
      }
   ]
}
"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let docs = ApiDocs::from(vec![ApiDoc {
            id: 2639696107074,
            system_name: "echo".into(),
            name: "Echo".into(),
            published: true,
            skip_swagger_validations: false,
            body: "{}".into(),
        }]);
        let result = serde_json::to_string_pretty(&docs);
        match result {
            Err(ref e) => println!("Error: {:#?}", e),
            _ => (),
        }
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
