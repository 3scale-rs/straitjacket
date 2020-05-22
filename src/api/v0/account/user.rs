use serde::{Deserialize, Serialize};
use straitjacket_macro::straitjacket;

pub type Metadata = crate::resources::Metadata;

#[straitjacket]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u64,
    account_id: Option<String>,
    state: String,
    role: String,
    username: String,
    email: String,
    extra_fields: Option<Vec<String>>,
}

endpoint! { EP_LIST_USERS, GET joining [ "/admin/api/accounts/", "/users.json" ] returning Users }
endpoint_test! { it_parses, EP_LIST_USERS, r##"{
   "users" : [
      {
         "user" : {
            "updated_at" : "2019-12-10T16:33:41+00:00",
            "email" : "goluser@flawedcode.org",
            "state" : "active",
            "role" : "admin",
            "links" : [
               {
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727",
                  "rel" : "account"
               },
               {
                  "rel" : "self",
                  "href" : "https://istiodevel-admin.3scale.net/admin/api/accounts/2445582851727/users/2445583035585"
               }
            ],
            "created_at" : "2019-12-10T16:33:23+00:00",
            "username" : "goluser",
            "id" : 2445583035585
         }
      }
   ]
}"## }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let users = Users::from(vec![
            User {
                id: 2445583035585,
                account_id: None,
                state: "active".into(),
                role: "admin".into(),
                username: "goluser".into(),
                email: "goluser@flawedcode.org".into(),
                extra_fields: None
            },
        ]);
        println!("{}", serde_json::to_string_pretty(&users).unwrap());
    }
}
