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
pub struct TargetDetail {
    /// url
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "criticality")]
    pub criticality: i64,
    #[serde(rename = "target_id", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "continuous_mode", skip_serializing_if = "Option::is_none")]
    pub continuous_mode: Option<bool>,
    #[serde(rename = "last_scan_date", skip_serializing_if = "Option::is_none")]
    pub last_scan_date: Option<String>,
    #[serde(rename = "last_scan_id", skip_serializing_if = "Option::is_none")]
    pub last_scan_id: Option<String>,
    #[serde(rename = "last_scan_session_id", skip_serializing_if = "Option::is_none")]
    pub last_scan_session_id: Option<String>,
    #[serde(rename = "last_scan_session_status", skip_serializing_if = "Option::is_none")]
    pub last_scan_session_status: Option<String>,
}

impl TargetDetail {
    pub fn new(address: String, _type: String, criticality: i64) -> TargetDetail {
        TargetDetail {
            address,
            description: None,
            _type,
            criticality,
            target_id: None,
            continuous_mode: None,
            last_scan_date: None,
            last_scan_id: None,
            last_scan_session_id: None,
            last_scan_session_status: None,
        }
    }
}

