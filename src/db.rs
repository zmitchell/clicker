use postgres::{Connection, TlsMode};
use tql::{PrimaryKey, ForeignKey};
use tql_macros::sql;


#[derive(SqlTable)]
struct Recitation {
    id: PrimaryKey,
    week: i32,
}

#[derive(SqlTable)]
struct Section {
    id: PrimaryKey,
    name: String,
}

#[derive(SqlTable)]
struct Student {
    id: PrimaryKey,
    sec_id: ForeignKey<Section>
}

#[derive(SqlTable)]
struct Clicker {
    id: PrimaryKey,
    address: String,
    student_id: ForeignKey<Student>
}

#[derive(SqlTable)]
struct Session {
    id: PrimaryKey,
    sec_id: ForeignKey<Section>,
    rec_id: ForeignKey<Recitation>
}

#[derive(SqlTable)]
struct Question {
    id: PrimaryKey,
    rec_id: ForeignKey<Recitation>,
    answer: Option<String>,
    alternate_answer: Option<String>,
    any_answer: Option<String>,
}

#[derive(SqlTable)]
struct Response {
    id: PrimaryKey,
    clicker_id: ForeignKey<Clicker>,
    session_id: ForeignKey<Session>,
    question_id: ForeignKey<Question>,
    first_answer: String,
    last_answer: String,
    first_time: String,
    last_time: String,
    attempts: i8,
}

#[derive(Debug)]
pub struct Gradebook {
    conn: Connection,
}

impl Gradebook {
    pub fn new() -> Self {
        let connection = Connection::connect("postgres://zmitchell@localhost:5432", TlsMode::None).unwrap();
        sql!(Recitation.create()).unwrap();
        sql!(Section.create()).unwrap();
        sql!(Student.create()).unwrap();
        sql!(Clicker.create()).unwrap();
        sql!(Session.create()).unwrap();
        sql!(Question.create()).unwrap();
        sql!(Response.create()).unwrap();
        Gradebook {
            conn: connection
        }
    }

    pub fn get_conn(&self) -> &Connection {
        return &self.conn
    }

    pub fn add_recitation_section(&self, name: &str) {
        let connection = self.get_conn();
        let _ = sql!(Section.insert(name = name)).unwrap();
    }

    pub fn add_recitation(&self, week: i32) {
        let connection = self.get_conn();
        let _ = sql!(Recitation.insert(week = week)).unwrap();
    }
}
