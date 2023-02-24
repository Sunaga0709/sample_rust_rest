#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// エラー時response用スキーマ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<models::ErrorError>,

}

impl Error {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Error {
        Error {
            error: None,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping error in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<models::ErrorError>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Error".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<models::ErrorError as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Error".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Error>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Error - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Error as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Error - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorError {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<models::ErrorErrorErrorsInner>>,

    /// HTTPステータス
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i32>,

    /// エラーメッセージ
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}

impl ErrorError {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ErrorError {
        ErrorError {
            errors: None,
            code: None,
            message: None,
        }
    }
}

/// Converts the ErrorError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping errors in query parameter serialization


            self.code.as_ref().map(|code| {
                vec![
                    "code".to_string(),
                    code.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                vec![
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub errors: Vec<Vec<models::ErrorErrorErrorsInner>>,
            pub code: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "errors" => return std::result::Result::Err("Parsing a container in this style is not supported in ErrorError".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorError {
            errors: intermediate_rep.errors.into_iter().next(),
            code: intermediate_rep.code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorErrorErrorsInner {
    /// ドメイン
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain: Option<String>,

    /// エラーコード
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    /// エラーメッセージ
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}

impl ErrorErrorErrorsInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ErrorErrorErrorsInner {
        ErrorErrorErrorsInner {
            domain: None,
            reason: None,
            message: None,
        }
    }
}

/// Converts the ErrorErrorErrorsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorErrorErrorsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.domain.as_ref().map(|domain| {
                vec![
                    "domain".to_string(),
                    domain.to_string(),
                ].join(",")
            }),


            self.reason.as_ref().map(|reason| {
                vec![
                    "reason".to_string(),
                    reason.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                vec![
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorErrorErrorsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorErrorErrorsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub domain: Vec<String>,
            pub reason: Vec<String>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorErrorErrorsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "domain" => intermediate_rep.domain.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorErrorErrorsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorErrorErrorsInner {
            domain: intermediate_rep.domain.into_iter().next(),
            reason: intermediate_rep.reason.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorErrorErrorsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorErrorErrorsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorErrorErrorsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorErrorErrorsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorErrorErrorsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorErrorErrorsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorErrorErrorsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UsersV1Get200ResponseInner {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "age")]
    pub age: u32,

    #[serde(rename = "email")]
    pub email: String,

}

impl UsersV1Get200ResponseInner {
    #[allow(clippy::new_without_default)]
    pub fn new(user_id: String, name: String, age: u32, email: String, ) -> UsersV1Get200ResponseInner {
        UsersV1Get200ResponseInner {
            user_id,
            name,
            age,
            email,
        }
    }
}

/// Converts the UsersV1Get200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UsersV1Get200ResponseInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("user_id".to_string()),
            Some(self.user_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("age".to_string()),
            Some(self.age.to_string()),


            Some("email".to_string()),
            Some(self.email.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UsersV1Get200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UsersV1Get200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub name: Vec<String>,
            pub age: Vec<u32>,
            pub email: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UsersV1Get200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "user_id" => intermediate_rep.user_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "age" => intermediate_rep.age.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UsersV1Get200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UsersV1Get200ResponseInner {
            user_id: intermediate_rep.user_id.into_iter().next().ok_or_else(|| "user_id missing in UsersV1Get200ResponseInner".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in UsersV1Get200ResponseInner".to_string())?,
            age: intermediate_rep.age.into_iter().next().ok_or_else(|| "age missing in UsersV1Get200ResponseInner".to_string())?,
            email: intermediate_rep.email.into_iter().next().ok_or_else(|| "email missing in UsersV1Get200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UsersV1Get200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UsersV1Get200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UsersV1Get200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UsersV1Get200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UsersV1Get200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UsersV1Get200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UsersV1Get200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

