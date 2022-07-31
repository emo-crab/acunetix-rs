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
pub struct UpdateInfo {
    #[serde(rename = "build_number")]
    pub build_number: String,
    #[serde(rename = "major_version")]
    pub major_version: String,
    #[serde(rename = "minor_version")]
    pub minor_version: String,
    #[serde(rename = "new_update")]
    pub new_update: bool,
    #[serde(rename = "update_status", skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

impl UpdateInfo {
    pub fn new(build_number: String, major_version: String, minor_version: String, new_update: bool) -> UpdateInfo {
        UpdateInfo {
            build_number,
            major_version,
            minor_version,
            new_update,
            update_status: None,
        }
    }
}


