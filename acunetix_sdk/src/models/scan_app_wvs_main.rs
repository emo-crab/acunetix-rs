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
pub struct ScanAppWvsMain {
    #[serde(rename = "web_scan_status", skip_serializing_if = "Option::is_none")]
    pub web_scan_status: Option<Box<crate::models::ScanAppWvsMainStatus>>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vulns", skip_serializing_if = "Option::is_none")]
    pub vulns: Option<Vec<crate::models::ScanAppVuln>>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::ScanAppMessage>>,
}

impl ScanAppWvsMain {
    pub fn new() -> ScanAppWvsMain {
        ScanAppWvsMain {
            web_scan_status: None,
            progress: None,
            duration: None,
            status: None,
            vulns: None,
            messages: None,
        }
    }
}

