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
pub struct LicenseExtra {
    #[serde(rename = "extra_std_target_count")]
    pub extra_std_target_count: i64,
    #[serde(rename = "extra_user_count", skip_serializing_if = "Option::is_none")]
    pub extra_user_count: Option<i64>,
    #[serde(rename = "can_create_new_std_target")]
    pub can_create_new_std_target: bool,
    #[serde(rename = "can_create_new_demo_target", skip_serializing_if = "Option::is_none")]
    pub can_create_new_demo_target: Option<bool>,
    #[serde(rename = "can_create_new_user", skip_serializing_if = "Option::is_none")]
    pub can_create_new_user: Option<bool>,
    #[serde(rename = "target_deletion_allowance")]
    pub target_deletion_allowance: serde_json::Value,
    #[serde(rename = "unique_std_target_count", skip_serializing_if = "Option::is_none")]
    pub unique_std_target_count: Option<i64>,
    #[serde(rename = "user_count")]
    pub user_count: i64,
}

impl LicenseExtra {
    pub fn new(extra_std_target_count: i64, can_create_new_std_target: bool, target_deletion_allowance: serde_json::Value, user_count: i64) -> LicenseExtra {
        LicenseExtra {
            extra_std_target_count,
            extra_user_count: None,
            can_create_new_std_target,
            can_create_new_demo_target: None,
            can_create_new_user: None,
            target_deletion_allowance,
            unique_std_target_count: None,
            user_count,
        }
    }
}


