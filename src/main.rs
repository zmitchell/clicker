#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate postgres;

mod db;
mod xml;

use db::Gradebook;
use xml::XmlSession;
use serde_xml_rs::deserialize;


fn main() {
    let contents: &str = include_str!("../L1709270831.xraw");
    let s_xml: XmlSession = deserialize(contents.as_bytes()).unwrap();
    let gb: Gradebook = Gradebook::new();
    gb.drop_tables();
    gb.create_tables();
    let (rec_id, sec_id) = gb.create_recitation();
    gb.ingest_responses(&s_xml, rec_id, sec_id);
    // gb.drop_tables();
}
