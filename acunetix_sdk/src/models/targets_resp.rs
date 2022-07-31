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
pub struct TargetsResp {
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<crate::models::TargetDetail>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
}

impl TargetsResp {
    pub fn new() -> TargetsResp {
        TargetsResp {
            targets: None,
            pagination: None,
        }
    }
}


