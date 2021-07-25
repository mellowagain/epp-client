use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppContactCheckResponse = EppObject<CommandResponse<ContactCheckResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheck {
    #[serde(rename = "$value")]
    pub id: StringValue,
    #[serde(rename = "avail")]
    pub available: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckDataItem {
    #[serde(rename = "id")]
    pub contact: ContactCheck,
    pub reason: Option<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckData {
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckDataItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResult {
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckData,
}
