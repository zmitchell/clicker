#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate rusqlite;

use std::convert::From;
use serde_xml_rs::deserialize;
use rusqlite::Connection;


#[derive(Deserialize,Debug)]
#[serde(rename = "ssn")]
struct XmlSession {
    #[serde(rename = "p")]
    pub questions: Vec<XmlQuestion>,
}

#[derive(Deserialize,Debug)]
#[serde(rename = "p")]
struct XmlQuestion {
    #[serde(rename = "idx")]
    pub number: String,

    #[serde(rename = "v")]
    pub responses: Vec<XmlResponse>,
}

#[derive(Deserialize,Debug)]
#[serde(rename = "v")]
struct XmlResponse {
    #[serde(rename = "fans")]
    pub first_answer: String,

    #[serde(rename = "ans")]
    pub final_answer: String,

    #[serde(rename = "tm")]
    pub first_answer_time: String,

    #[serde(rename = "fanst")]
    pub final_answer_time: String,

    #[serde(rename = "id")]
    pub clicker: String,

    #[serde(rename = "att")]
    pub attempts: String,
}


enum Tables {
    Recitation,
    Student,
    Section,
    Clicker,
    Session,
    Question,
    Response,
}


impl<'a> From<&'a str> for Tables {
    fn from(item: &str) -> Self {
        match item {
            "Recitation" => Tables::Recitation,
            "Student" => Tables::Student,
            "Section" => Tables::Section,
            "Clicker" => Tables::Clicker,
            "Session" => Tables::Session,
            "Question" => Tables::Question,
            "Response" => Tables::Response,
            val => panic!("Could not convert {:?} into `Tables` variant", val)
        }
    }
}


impl From<String> for Tables {
    fn from(item: String) -> Self {
        let s: &str = item.as_str();
        Self::from(s)
    }
}


struct Gradebook {
    conn: Connection,
}


impl Gradebook {
    fn new() -> Gradebook {
        let conn: Connection = Connection::open_in_memory().unwrap();
        Gradebook {
            conn: conn,
        }
    }

    fn create_tables(&self) {
        let sql: &str = include_str!("../create_schema.sql");
        self.conn.execute(sql, &[]).unwrap();
    }

    fn new_recitation(&self, num: u8) {
        self.conn.execute("INSERT INTO Recitation (recitation_number)
            VALUES (?1)", &[&num]).unwrap();
    }
}


fn main() {
    let contents: &str = include_str!("../L1709270831.xraw");
    let session: XmlSession = deserialize(contents.as_bytes()).unwrap();
    let gb: Gradebook = Gradebook::new();
    gb.create_tables();
    gb.new_recitation(1);
}
