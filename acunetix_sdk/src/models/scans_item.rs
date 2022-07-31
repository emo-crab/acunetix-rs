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
pub struct ScansItem {
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<crate::models::Target>>,
    #[serde(rename = "target_id", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "current_session", skip_serializing_if = "Option::is_none")]
    pub current_session: Option<Box<crate::models::ScanDetailCurrentSession>>,
    #[serde(rename = "manual_intervention", skip_serializing_if = "Option::is_none")]
    pub manual_intervention: Option<bool>,
    #[serde(rename = "next_run", skip_serializing_if = "Option::is_none")]
    pub next_run: Option<String>,
    #[serde(rename = "profile_id", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "profile_name", skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "report_template_id", skip_serializing_if = "Option::is_none")]
    pub report_template_id: Option<String>,
    #[serde(rename = "scan_id", skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::Schedual>>,
    /// url
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "criticality")]
    pub criticality: i64,
}

impl ScansItem {
    pub fn new(address: String, _type: String, criticality: i64) -> ScansItem {
        ScansItem {
            target: None,
            target_id: None,
            current_session: None,
            manual_intervention: None,
            next_run: None,
            profile_id: None,
            profile_name: None,
            report_template_id: None,
            scan_id: None,
            schedule: None,
            address,
            description: None,
            _type,
            criticality,
        }
    }
}


