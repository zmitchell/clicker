# Clicker - Grade student responses with Rust

Teachers in higher education are able to poll students in real-time using dedicated software and hardware. A teacher may present a question on a board, projector, etc, open a poll, and ask the students to submit an answer. Students respond using remotes, and a base station in the classroom records the responses. The responses are saved and later graded using compatible software. The remotes, base station, polling software, and grading software are typically all provided by the same company.

The courses I teach all use the iClicker polling and grading software. The grading software doesn't always mesh well with the structure of the course I teach, so I've started working on my own solution.

# Goals
The goals of this project are as follows:
* Allow several sections of a course to be graded with one set of answers
* Reduce the overall friction of the grading process
* Quickly and easily generate text files for uploading to a gradebook

# Legal Stuff
* License: MIT License
* This project is in no way affiliated with iClicker or Macmillan Learning
