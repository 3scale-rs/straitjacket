use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub mod feature;
pub mod plan;
pub mod user;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    id: u64,
    state: String,
    org_name: String,
    extra_fields: Option<Vec<String>>,
    monthly_billing_enabled: bool,
    monthly_charging_enabled: bool,
    credit_card_stored: bool,
    plans: Option<Vec<plan::Plan>>,
}

endpoint! { EP_LIST_ACCOUNTS, GET joining [ "/admin/api/accounts.json" ] returning Accounts }
endpoint_test! { it_parses, EP_LIST_ACCOUNTS, r##"{
   "accounts" : [
      {
         "account" : {
            "updated_at" : "2018-07-06T12:30:23+01:00",
            "state" : "approved",
            "monthly_charging_enabled" : true,
            "org_name" : "Developer",
            "created_at" : "2018-07-06T12:30:22+01:00",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514",
                  "rel" : "self"
               },
               {
                  "rel" : "users",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582571514/users"
               }
            ],
            "id" : 2445582571514,
            "credit_card_stored" : false,
            "monthly_billing_enabled" : true
         }
      },
      {
         "account" : {
            "created_at" : "2018-10-02T15:24:36+01:00",
            "links" : [
               {
                  "rel" : "self",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582616945"
               },
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582616945/users",
                  "rel" : "users"
               }
            ],
            "id" : 2445582616945,
            "monthly_billing_enabled" : true,
            "credit_card_stored" : false,
            "state" : "approved",
            "updated_at" : "2018-10-02T15:24:49+01:00",
            "org_name" : "ostia",
            "monthly_charging_enabled" : true
         }
      },
      {
         "account" : {
            "monthly_charging_enabled" : true,
            "org_name" : "IstioUsers",
            "updated_at" : "2019-03-31T02:54:19+01:00",
            "state" : "approved",
            "monthly_billing_enabled" : false,
            "credit_card_stored" : false,
            "id" : 2445582716891,
            "created_at" : "2019-03-31T02:53:53+01:00",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891",
                  "rel" : "self"
               },
               {
                  "rel" : "users",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582716891/users"
               }
            ]
         }
      },
      {
         "account" : {
            "org_name" : "test-jSXw5FGy57A",
            "monthly_charging_enabled" : true,
            "state" : "created",
            "updated_at" : "2019-06-18T18:53:05+01:00",
            "monthly_billing_enabled" : true,
            "credit_card_stored" : false,
            "id" : 2445582759096,
            "links" : [
               {
                  "rel" : "self",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759096"
               },
               {
                  "rel" : "users",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759096/users"
               }
            ],
            "created_at" : "2019-06-18T18:53:05+01:00"
         }
      },
      {
         "account" : {
            "monthly_charging_enabled" : true,
            "org_name" : "test-N-ucDECvz4s",
            "updated_at" : "2019-06-18T18:58:08+01:00",
            "state" : "created",
            "id" : 2445582759102,
            "credit_card_stored" : false,
            "monthly_billing_enabled" : true,
            "links" : [
               {
                  "rel" : "self",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759102"
               },
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759102/users",
                  "rel" : "users"
               }
            ],
            "created_at" : "2019-06-18T18:58:08+01:00"
         }
      },
      {
         "account" : {
            "id" : 2445582759106,
            "monthly_billing_enabled" : true,
            "credit_card_stored" : false,
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759106",
                  "rel" : "self"
               },
               {
                  "rel" : "users",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759106/users"
               }
            ],
            "created_at" : "2019-06-18T19:04:03+01:00",
            "org_name" : "test-QrRcdVg_Nc8",
            "monthly_charging_enabled" : true,
            "state" : "created",
            "updated_at" : "2019-06-18T19:04:03+01:00"
         }
      },
      {
         "account" : {
            "credit_card_stored" : false,
            "monthly_billing_enabled" : true,
            "id" : 2445582759112,
            "created_at" : "2019-06-18T19:19:47+01:00",
            "links" : [
               {
                  "rel" : "self",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759112"
               },
               {
                  "rel" : "users",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759112/users"
               }
            ],
            "monthly_charging_enabled" : true,
            "org_name" : "test-0qr4NMhCOLc",
            "updated_at" : "2019-06-18T19:19:47+01:00",
            "state" : "created"
         }
      },
      {
         "account" : {
            "monthly_charging_enabled" : true,
            "org_name" : "test-uuGB5uHbujw",
            "updated_at" : "2019-06-18T19:20:15+01:00",
            "state" : "created",
            "credit_card_stored" : false,
            "monthly_billing_enabled" : true,
            "id" : 2445582759114,
            "created_at" : "2019-06-18T19:20:15+01:00",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759114",
                  "rel" : "self"
               },
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759114/users",
                  "rel" : "users"
               }
            ]
         }
      },
      {
         "account" : {
            "created_at" : "2019-06-18T19:21:46+01:00",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759115",
                  "rel" : "self"
               },
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582759115/users",
                  "rel" : "users"
               }
            ],
            "id" : 2445582759115,
            "monthly_billing_enabled" : true,
            "credit_card_stored" : false,
            "state" : "created",
            "updated_at" : "2019-06-18T19:21:46+01:00",
            "org_name" : "test-nWG4mvQl_z0",
            "monthly_charging_enabled" : true
         }
      },
      {
         "account" : {
            "id" : 2445582851727,
            "monthly_billing_enabled" : true,
            "credit_card_stored" : false,
            "created_at" : "2019-12-10T16:33:23+00:00",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727",
                  "rel" : "self"
               },
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/users",
                  "rel" : "users"
               }
            ],
            "monthly_charging_enabled" : true,
            "org_name" : "Org",
            "updated_at" : "2019-12-10T16:33:41+00:00",
            "state" : "approved"
         }
      }
   ]
}"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let accounts = Accounts::from(vec![
            Account {
                id: 2445583035585,
                state: "created".into(),
                extra_fields: None,
                monthly_billing_enabled: true,
                monthly_charging_enabled: true,
                credit_card_stored: false,
                plans: None,
                org_name: "test-nWG4mvQl_z0".into(),
            },
        ]);
        println!("{}", serde_json::to_string_pretty(&accounts).unwrap());
    }
}
