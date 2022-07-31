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
pub struct ScanAppHost {
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "is_starting_host", skip_serializing_if = "Option::is_none")]
    pub is_starting_host: Option<bool>,
    #[serde(rename = "web_scan_status", skip_serializing_if = "Option::is_none")]
    pub web_scan_status: Option<Box<crate::models::ScanAppHostWebScanStatus>>,
    #[serde(rename = "target_info", skip_serializing_if = "Option::is_none")]
    pub target_info: Option<Box<crate::models::ScanAppHostTargetInfo>>,
    #[serde(rename = "aborted", skip_serializing_if = "Option::is_none")]
    pub aborted: Option<bool>,
    #[serde(rename = "aborted_reason", skip_serializing_if = "Option::is_none")]
    pub aborted_reason: Option<String>,
    #[serde(rename = "external_hosts", skip_serializing_if = "Option::is_none")]
    pub external_hosts: Option<Vec<String>>,
}

impl ScanAppHost {
    pub fn new() -> ScanAppHost {
        ScanAppHost {
            host: None,
            is_starting_host: None,
            web_scan_status: None,
            target_info: None,
            aborted: None,
            aborted_reason: None,
            external_hosts: None,
        }
    }
}


