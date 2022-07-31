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
pub struct Profiles {
    #[serde(rename = "scanning_profiles", skip_serializing_if = "Option::is_none")]
    pub scanning_profiles: Option<Vec<crate::models::Profile>>,
}

impl Profiles {
    pub fn new() -> Profiles {
        Profiles {
            scanning_profiles: None,
        }
    }
}


