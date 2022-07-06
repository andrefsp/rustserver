#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateUserRequest {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<serde_json::Value>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<models::User>,

    #[serde(rename = "task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<models::Task>,

}

impl CreateUserRequest {
    pub fn new() -> CreateUserRequest {
        CreateUserRequest {
            type_: None,
            user: None,
            task: None,
        }
    }
}

/// Converts the CreateUserRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateUserRequest {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping type in query parameter serialization

        // Skipping user in query parameter serialization

        // Skipping task in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateUserRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateUserRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<serde_json::Value>,
            pub user: Vec<models::User>,
            pub task: Vec<models::Task>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateUserRequest".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "user" => intermediate_rep.user.push(<models::User as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "task" => intermediate_rep.task.push(<models::Task as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateUserRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateUserRequest {
            type_: intermediate_rep.type_.into_iter().next(),
            user: intermediate_rep.user.into_iter().next(),
            task: intermediate_rep.task.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateUserRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateUserRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateUserRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateUserRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateUserRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateUserRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateUserRequest - {}",
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
pub struct Task {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<isize>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "epic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub epic: Option<String>,

}

impl Task {
    pub fn new() -> Task {
        Task {
            id: None,
            description: None,
            epic: None,
        }
    }
}

/// Converts the Task value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Task {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref epic) = self.epic {
            params.push("epic".to_string());
            params.push(epic.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Task value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Task {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<isize>,
            pub description: Vec<String>,
            pub epic: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Task".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "epic" => intermediate_rep.epic.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Task".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Task {
            id: intermediate_rep.id.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            epic: intermediate_rep.epic.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Task> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Task>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Task>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Task - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Task> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Task as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Task - {}",
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
pub struct User {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            email: None,
            username: None,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for User {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub email: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            id: intermediate_rep.id.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

