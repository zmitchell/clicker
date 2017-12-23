CREATE TABLE Recitation (
    recitation_id INTEGER NOT NULL,
    recitation_number INTEGER NOT NULL,
    PRIMARY KEY (recitation_id)
);

CREATE TABLE Section (
    section_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (section_id)
);

CREATE TABLE Student (
    student_id INTEGER NOT NULL,
    section_id INTEGER,
    PRIMARY KEY (student_id),
    FOREIGN KEY (section_id) REFERENCES Section(section_id)
);

CREATE TABLE Session (
    session_id INTEGER NOT NULL,
    section_id INTEGER,
    session_name TEXT NOT NULL,
    recitation_id INTEGER,
    PRIMARY KEY (session_id),
    FOREIGN KEY (section_id) REFERENCES Section(section_id),
    FOREIGN KEY (recitation_id) REFERENCES Recitation(recitation_id)
);

CREATE TABLE Clicker (
    clicker_id INTEGER NOT NULL,
    address TEXT NOT NULL,
    student_id INTEGER,
    PRIMARY KEY (clicker_id),
    FOREIGN KEY (student_id) REFERENCES Student(student_id)
);

CREATE TABLE Question (
    question_id INTEGER NOT NULL,
    recitation_id INTEGER,
    correct_answer TEXT NOT NULL,
    alternate_answer TEXT,
    any_answer INTEGER,
    PRIMARY KEY (question_id),
    FOREIGN KEY (recitation_id) REFERENCES Recitation(recitation_id)
);

CREATE TABLE Response (
    response_id INTEGER NOT NULL,
    clicker_id INTEGER,
    question_id INTEGER,
    session_id INTEGER,
    first_answer TEXT NOT NULL,
    final_answer TEXT NOT NULL,
    first_time TEXT NOT NULL,
    final_time TEXT NOT NULL,
    attempts INTEGER NOT NULL,
    PRIMARY KEY (response_id),
    FOREIGN KEY (clicker_id) REFERENCES Clicker(clicker_id),
    FOREIGN KEY (question_id) REFERENCES Question(question_id),
    FOREIGN KEY (session_id) REFERENCES Session(session_id)
);
