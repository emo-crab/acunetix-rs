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
pub struct License {
    #[serde(rename = "access")]
    pub access: bool,
    #[serde(rename = "actived", skip_serializing_if = "Option::is_none")]
    pub actived: Option<bool>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "expired")]
    pub expired: bool,
    #[serde(rename = "expires")]
    pub expires: String,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<crate::models::LicenseLimit>>,
    #[serde(rename = "license_key", skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(rename = "maintenance_expired", skip_serializing_if = "Option::is_none")]
    pub maintenance_expired: Option<bool>,
    #[serde(rename = "maintenance_expires", skip_serializing_if = "Option::is_none")]
    pub maintenance_expires: Option<String>,
    #[serde(rename = "product_code", skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
}

impl License {
    pub fn new(access: bool, email: String, expired: bool, expires: String) -> License {
        License {
            access,
            actived: None,
            email,
            expired,
            expires,
            features: None,
            limits: None,
            license_key: None,
            maintenance_expired: None,
            maintenance_expires: None,
            product_code: None,
        }
    }
}


