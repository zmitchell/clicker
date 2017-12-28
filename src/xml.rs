#[derive(Deserialize,Debug)]
#[serde(rename = "ssn")]
pub struct XmlSession {
    #[serde(rename = "p")]
    pub questions: Vec<XmlQuestion>,
}

#[derive(Deserialize,Debug)]
#[serde(rename = "p")]
pub struct XmlQuestion {
    #[serde(rename = "idx")]
    pub number: String,

    #[serde(rename = "v")]
    pub responses: Vec<XmlResponse>,
}

#[derive(Deserialize,Debug)]
#[serde(rename = "v")]
pub struct XmlResponse {
    #[serde(rename = "fans")]
    pub first_ans: String,

    #[serde(rename = "ans")]
    pub final_ans: String,

    #[serde(rename = "tm")]
    pub first_time: String,

    #[serde(rename = "fanst")]
    pub final_time: String,

    #[serde(rename = "id")]
    pub address: String,

    #[serde(rename = "att")]
    pub attempts: String,
}
