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
pub struct TargetConfigLogin {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<crate::models::TargetConfigLoginCredentials>>,
}

impl TargetConfigLogin {
    pub fn new() -> TargetConfigLogin {
        TargetConfigLogin {
            kind: None,
            credentials: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "automatic")]
    Automatic,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::None
    }
}
