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
pub struct Schedual {
    #[serde(rename = "disable")]
    pub disable: bool,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "time_sensitive", skip_serializing_if = "Option::is_none")]
    pub time_sensitive: Option<bool>,
}

impl Schedual {
    pub fn new(disable: bool) -> Schedual {
        Schedual {
            disable,
            start_date: None,
            time_sensitive: None,
        }
    }
}


