use postgres::{Connection, TlsMode};
use xml::{XmlResponse, XmlSession};


#[derive(Debug)]
pub struct Recitation {
    pub id: i32,
    pub week: i32,
}


#[derive(Debug)]
pub struct Section {
    pub id: i32,
    pub name: String,
}


#[derive(Debug)]
pub struct Student {
    pub id: i32,
    pub sec_id: Option<i32>,
}


#[derive(Debug)]
pub struct Clicker {
    pub id: i32,
    pub address: String,
    pub student_id: Option<i32>,
}


#[derive(Debug)]
pub struct Session {
    pub id: i32,
    pub sec_id: Option<i32>,
    pub rec_id: Option<i32>,
}


#[derive(Debug)]
pub struct Question {
    pub id: i32,
    pub num: i32,
    pub rec_id: Option<i32>,
    pub ans: Option<String>,
    pub alt_ans: Option<String>,
    pub any_ans: bool,
}


#[derive(Debug)]
pub struct Response {
    pub id: i32,
    pub clicker_id: Option<i32>,
    pub session_id: Option<i32>,
    pub question_id: Option<i32>,
    pub first_answer: String,
    pub last_answer: String,
    pub first_time: String,
    pub last_time: String,
    pub attempts: i8,
}


#[derive(Debug)]
pub struct QuestionInput {
    pub num: i32,
    pub ans: Option<String>,
    pub alt_ans: Option<String>,
    pub any_ans: bool,
}


#[derive(Debug)]
pub struct Gradebook {
    conn: Connection,
}


impl Gradebook {
    pub fn new() -> Self {
        let connection = Connection::connect("postgres://zmitchell@localhost:5432", TlsMode::None).unwrap();
        Gradebook {
            conn: connection
        }
    }

    pub fn get_conn(&self) -> &Connection {
        return &self.conn
    }

    pub fn create_tables(&self) {
        &self.conn.execute("CREATE TABLE recitation (
            id SERIAL,
            week INTEGER NOT NULL,
            PRIMARY KEY (id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE section (
            id SERIAL,
            name CHAR(4) NOT NULL,
            PRIMARY KEY (id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE student (
            id SERIAL,
            sec_id INTEGER,
            PRIMARY KEY (id),
            FOREIGN KEY (sec_id) REFERENCES section(id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE session (
            id SERIAL,
            sec_id INTEGER,
            rec_id INTEGER,
            PRIMARY KEY (id),
            FOREIGN KEY (sec_id) REFERENCES section(id),
            FOREIGN KEY (rec_id) REFERENCES recitation(id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE clicker (
            id SERIAL,
            address CHAR(9) NOT NULL,
            student_id INTEGER,
            PRIMARY KEY (id),
            FOREIGN KEY (student_id) REFERENCES student(id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE question (
            id SERIAL,
            num INTEGER NOT NULL,
            rec_id INTEGER,
            ans CHAR(1),
            alt_ans CHAR(1),
            any_ans BOOLEAN NOT NULL,
            PRIMARY KEY (id),
            FOREIGN KEY (rec_id) REFERENCES recitation(id)
            )", &[]).unwrap();
        &self.conn.execute("CREATE TABLE response (
            id SERIAL,
            clicker_id INTEGER,
            q_id INTEGER,
            session_id INTEGER,
            first_ans CHAR(1),
            final_ans CHAR(1),
            first_time VARCHAR(10),
            final_time VARCHAR(10),
            attempts INTEGER NOT NULL,
            PRIMARY KEY (id),
            FOREIGN KEY (clicker_id) REFERENCES clicker(id),
            FOREIGN KEY (q_id) REFERENCES question(id),
            FOREIGN KEY (session_id) REFERENCES session(id)
            )", &[]).unwrap();
    }

    pub fn drop_tables(&self) {
        let conn = self.get_conn();
        conn.execute("DROP TABLE response", &[]).unwrap();
        conn.execute("DROP TABLE question", &[]).unwrap();
        conn.execute("DROP TABLE session", &[]).unwrap();
        conn.execute("DROP TABLE clicker", &[]).unwrap();
        conn.execute("DROP TABLE student", &[]).unwrap();
        conn.execute("DROP TABLE recitation", &[]).unwrap();
        conn.execute("DROP TABLE section", &[]).unwrap();
    }

    pub fn make_section(&self, name: &str) {
        &self.conn.execute("INSERT INTO section(name) VALUES ($1)", &[&name]).unwrap();
    }

    pub fn get_section_id_by_name(&self, name: &str) -> i32 {
        let rows = &self.conn.query("SELECT * FROM section WHERE name = $1", &[&name]).unwrap();
        let id: i32 = rows.get(0).get("id");
        return id
    }

    pub fn get_section_by_id(&self, id: i32) -> Section {
        let rows = &self.conn.query("SELECT * FROM section WHERE id = $1", &[&id]).unwrap();
        let sec = Section {
            id: rows.get(0).get("id"),
            name: rows.get(0).get("name"),
        };
        sec
    }

    pub fn make_recitation_from_week(&self, rec_week: i32) {
        &self.conn.execute("INSERT INTO recitation(week) VALUES ($1)", &[&rec_week]).unwrap();
    }

    pub fn get_recitation_id_by_week(&self, rec_week: i32) -> i32 {
        let rows = &self.conn.query("SELECT * FROM recitation WHERE week = $1", &[&rec_week]).unwrap();
        let id: i32 = rows.get(0).get("id");
        id
    }

    pub fn get_recitation_by_id(&self, id: i32) -> Recitation {
        let rows = &self.conn.query("SELECT * FROM recitation WHERE id = $1", &[&id]).unwrap();
        let rec = Recitation {
            id: rows.get(0).get("id"),
            week: rows.get(0).get("week"),
        };
        rec
    }

    pub fn add_clicker_if_not_exists(&self, addr: &str) {
        let rows = &self.conn.query("SELECT * FROM clicker WHERE address = $1", &[&addr]).unwrap();
        if rows.len() == 0 {
            &self.conn.execute("INSERT INTO clicker(address) VALUES ($1)", &[&addr]).unwrap();
        } else {
            return
        }
    }

    pub fn get_clicker_id_by_address(&self, addr: &str) -> i32 {
        let rows = &self.conn.query("SELECT * FROM clicker WHERE address = $1",
            &[&addr]).unwrap();
        let id: i32 = rows.get(0).get("id");
        id
    }

    pub fn make_session(&self, rec_id: i32, sec_id: i32) {
        &self.conn.execute("INSERT INTO session(rec_id, sec_id) VALUES ($1, $2)",
            &[&Some(rec_id), &Some(sec_id)]).unwrap();
    }

    pub fn get_session(&self, rec_id: i32, sec_id: i32) -> i32 {
        let rows = &self.conn.query("SELECT * FROM session WHERE rec_id = $1 AND sec_id = $2",
            &[&rec_id, &sec_id]).unwrap();
        let id: i32 = rows.get(0).get("id");
        id
    }

    pub fn add_questions_to_recitation(&self, rec_id: i32, inputs: Vec<QuestionInput>) {
        for q in inputs.iter() {
            let num = q.num;
            let ans = q.ans.clone();
            let alt_ans = q.alt_ans.clone();
            let any_ans = q.any_ans;
            &self.conn.execute("INSERT INTO question(rec_id, num, ans, alt_ans, any_ans) VALUES ($1, $2, $3, $4, $5)",
                &[&Some(rec_id), &num, &Some(ans), &Some(alt_ans), &any_ans]).unwrap();
        }
    }

    pub fn insert_response(&self, click_id: i32, q_id: i32, sess_id: i32, r_xml: &XmlResponse) {
        let clicker_id: Option<i32> = Some(click_id);
        let question_id: Option<i32> = Some(q_id);
        let session_id: Option<i32> = Some(sess_id);
        let first_ans: &str = &r_xml.first_ans;
        let final_ans: &str = &r_xml.final_ans;
        let first_time: &str = &r_xml.first_time;
        let final_time: &str = &r_xml.final_time;
        let attempts: i32 = r_xml.attempts.parse().unwrap_or(0);
        &self.conn.execute("INSERT INTO
            response(clicker_id, q_id, session_id, first_ans, final_ans, first_time, final_time, attempts)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&clicker_id, &question_id, &session_id, &first_ans, &final_ans, &first_time, &final_time, &attempts])
            .unwrap();
    }

    pub fn ingest_responses(&self, session: &XmlSession, rec_id: i32, sec_id: i32) {
        self.make_session(rec_id, sec_id);
        let session_id = self.get_session(rec_id, sec_id);
        let questions = &self.conn.query("SELECT * FROM question WHERE rec_id = $1", &[&rec_id]).unwrap();
        let mut q_ids: Vec<i32> = Vec::new();
        for q in questions.iter() {
            q_ids.push(q.get("id"));
        }
        for (q_xml, q_id) in session.questions.iter().zip(q_ids.iter()) {
            for r_xml in q_xml.responses.iter() {
                self.add_clicker_if_not_exists(&r_xml.address);
                let clicker_id = self.get_clicker_id_by_address(&r_xml.address);
                self.insert_response(clicker_id, *q_id, session_id, r_xml);
            }
        }
    }

    pub fn create_recitation(&self) -> (i32, i32) {
        self.make_recitation_from_week(1);
        let rec_id = self.get_recitation_id_by_week(1);
        self.make_section("R006");
        let sec_id = self.get_section_id_by_name("R006");
        let questions: Vec<QuestionInput> = vec!(
            QuestionInput {
                num: 1,
                ans: Some("A".into()),
                alt_ans: Some("B".into()),
                any_ans: false,
            },
            QuestionInput {
                num: 2,
                ans: Some("C".into()),
                alt_ans: None,
                any_ans: false,
            },
            QuestionInput {
                num: 3,
                ans: None,
                alt_ans: None,
                any_ans: true,
            },
            QuestionInput {
                num: 4,
                ans: Some("D".into()),
                alt_ans: None,
                any_ans: false,
            },
            QuestionInput {
                num: 5,
                ans: Some("E".into()),
                alt_ans: None,
                any_ans: false,
            });
        self.add_questions_to_recitation(rec_id, questions);
        (rec_id, sec_id)
    }
}
