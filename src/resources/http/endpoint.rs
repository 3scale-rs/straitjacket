use super::path_builder::{ParameterQuantifier, PathBuilder};
use http::Method;
use serde::{de::DeserializeOwned, Deserialize};

pub struct Endpoint<'a, 's, M> {
    method: http::Method,
    path_builder: PathBuilder<'a, 's>,
    datatype: std::marker::PhantomData<M>,
}

impl<'a, 's, M> Endpoint<'a, 's, M> {
    #[cfg(const_fn)]
    pub const fn new(
        method: Method,
        segments: &'a [&'s str],
        quantifier: ParameterQuantifier,
    ) -> Self {
        Endpoint {
            method,
            path_builder: PathBuilder::new(segments, quantifier),
            datatype: std::marker::PhantomData,
        }
    }

    #[cfg(not(const_fn))]
    pub fn new(method: Method, segments: &'a [&'s str], quantifier: ParameterQuantifier) -> Self {
        Endpoint {
            method,
            path_builder: PathBuilder::new(segments, quantifier),
            datatype: std::marker::PhantomData,
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    fn path_builder(&self) -> &PathBuilder<'a, 's> {
        &self.path_builder
    }

    pub fn path(&self, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        self.path_builder().build(args)
    }

    pub fn accepted_parameters(&self) -> usize {
        self.path_builder().accepted_parameters()
    }
}

impl<'a, 's, M: DeserializeOwned> Endpoint<'a, 's, M> {
    pub fn parse_reader(&self, r: impl std::io::Read) -> Result<M, Box<dyn std::error::Error>> {
        let data = serde_json::from_reader(r)?;
        Ok(data)
    }
}

impl<'a, 's, 'de, M: Deserialize<'de>> Endpoint<'a, 's, M> {
    pub fn parse_str(&self, s: &'de str) -> Result<M, Box<dyn std::error::Error>> {
        let data = serde_json::from_str(s)?;
        Ok(data)
    }
}

macro_rules! endpoint_test {
    { $name:ident, $endpoint:path, $response:expr } => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let response = $response;
            let object = $endpoint.parse_str(&response);
            match object {
              Err(ref e) => println!("Error: {:#?}", e),
              _ => (),
            }
            assert!(object.is_ok());
            let object = object.unwrap();
            println!("PARSED:\n{:#?}", object);
        }
    };
}

macro_rules! endpoint {
    { $endpoint:ident, $method:expr, $paramjoin:path, [ $($segments:expr),+ ] returning $object:ty } => {
        use crate::resources::http::endpoint::Endpoint;
        use crate::resources::http::path_builder::ParameterQuantifier;

        #[cfg(const_fn)]
        pub const $endpoint: Endpoint<'_, '_, $object> = Endpoint::new($method, &[$($segments),+], $paramjoin);

        #[cfg(not(const_fn))]
        lazy_static::lazy_static! {
          pub static ref $endpoint: Endpoint<'static, 'static, $object> = Endpoint::new($method, &[$($segments),+], $paramjoin);
        }
    };
    { $endpoint:ident, $method:expr, joining $($body:tt)+ } => {
        endpoint! { $endpoint, $method, ParameterQuantifier::JoiningSegments, $($body)+ }
    };
    { $endpoint:ident, $method:expr, pairing $($body:tt)+ } => {
        endpoint! { $endpoint, $method, ParameterQuantifier::PairingSegments, $($body)+ }
    };
    { $endpoint:ident, GET $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, PUT $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, PATCH $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, POST $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, DELETE $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, HEAD $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
    { $endpoint:ident, OPTIONS $($body:tt)+ } => {
        endpoint! { $endpoint, http::Method::GET, $($body)+ }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    endpoint! { EPK, GET joining [ "/products/", "/properties" ] returning crate::proxy::mapping_rules::MappingRules }
    endpoint_test! { it_parses2, EPK, r##"{
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
}"## }

    #[test]
    fn it_creates_a_path_by_joining_args() {
        let ep = Endpoint::<'_, '_, crate::proxy::mapping_rules::MappingRules>::new(
            Method::GET,
            &["/products/", "/properties"],
            ParameterQuantifier::JoiningSegments,
        );
        let args = vec!["id123"];
        let path = ep.path(&args).unwrap();
        assert_eq!(path, "/products/id123/properties");
    }

    #[test]
    fn it_creates_a_path_by_joining_args_plus_a_final_arg() {
        let ep = Endpoint::<'_, '_, crate::proxy::mapping_rules::MappingRules>::new(
            Method::GET,
            &["/category/", "/properties/"],
            ParameterQuantifier::PairingSegments,
        );
        let args = vec!["products", "id123"];
        let path = ep.path(&args).unwrap();
        assert_eq!(path, "/category/products/properties/id123");
    }

    #[test]
    fn it_parses() {
        let ep = Endpoint::<'_, '_, crate::proxy::mapping_rules::MappingRules>::new(
            Method::GET,
            &["/category/", "/properties/"],
            ParameterQuantifier::PairingSegments,
        );
        let response = r##"{
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
        let object = ep.parse_str(&response);
        assert!(object.is_ok());
        let object = object.unwrap();
        println!("PARSED:\n{:#?}", object);
    }
}
