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
pub struct Me {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "firstname")]
    pub firstname: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "su")]
    pub su: bool,
}

impl Me {
    pub fn new(email: String, enabled: bool, firstname: String, user_id: String, su: bool) -> Me {
        Me {
            email,
            enabled,
            firstname,
            user_id,
            su,
        }
    }
}


