/*
 * Acunetix12 client api
 *
 * Acunetix12 client api [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/). 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponse {
    pub fn new() -> ApiResponse {
        ApiResponse {
            code: None,
            _type: None,
            message: None,
        }
    }
}


