#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate postgres;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;

mod db;
mod ui;
mod xml;

use serde_xml_rs::deserialize;
use std::process;
use relm::Widget;

use db::Gradebook;
use xml::XmlSession;
use ui::Application;


fn main() {
    // let contents: &str = include_str!("../L1709270831.xraw");
    // let s_xml: XmlSession = deserialize(contents.as_bytes()).unwrap();
    let gb = Gradebook::new();
    gb.drop_tables();
    gb.create_tables();

    Application::run(()).unwrap();
}
