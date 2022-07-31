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
pub struct TargetConfigCustomCookies {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "cookie", skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
}

impl TargetConfigCustomCookies {
    pub fn new() -> TargetConfigCustomCookies {
        TargetConfigCustomCookies {
            url: None,
            cookie: None,
        }
    }
}


