/*
 * AWVS12 client api
 *
 * Awvs12 client api [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/). 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanAppHostTargetInfo {
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "responsive", skip_serializing_if = "Option::is_none")]
    pub responsive: Option<bool>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

impl ScanAppHostTargetInfo {
    pub fn new() -> ScanAppHostTargetInfo {
        ScanAppHostTargetInfo {
            os: None,
            responsive: None,
            server: None,
        }
    }
}


