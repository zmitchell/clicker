use gtk;
use gtk::prelude::*;
use gtk::{Orientation, Inhibit};
use relm::Widget;
use relm_attributes::widget;

use std::collections::HashMap;

use self::QuestionMsg::*;


#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum Checkbox {
    A,
    B,
    C,
    D,
    E,
    Any
}


#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum AnswerChoice {
    A,
    B,
    C,
    D,
    E,
}


#[derive(Msg)]
pub enum QuestionMsg {
    CheckboxChanged(Checkbox, bool),
    UpdateChecked(u32, Checkbox, bool),
    Clear,
    Submit,
    SetLabel(String),
}


#[derive(Debug)]
pub struct QuestionModel {
    pub label: String,
    pub any: bool,
}


pub struct QuestionGridModel {
    pub checked: HashMap<(u32, Checkbox), bool>,
    pub answers: Vec<Vec<AnswerChoice>>,
}


// #[derive(Msg)]
// pub enum SectionMsg {
//     // empty for now
// }


// pub struct SectionModel {
//     pub sections: Vec<String>,
//     pub selected_for_removal: Option<String>,
// }


#[derive(Msg)]
pub enum ClickerMsg {
    Quit,
}


pub struct ClickerModel {
    // empty for now
}


#[widget]
impl Widget for Question {
    fn model() -> QuestionModel {
        QuestionModel {
            label: String::from(""),
            any: false,
        }
    }

    fn update(&mut self, event: QuestionMsg) {
        match event {
            CheckboxChanged(checkbox, state) => {
                match checkbox {
                    Checkbox::Any => {
                        self.model.any = state;
                    },
                    _ => {}
                }
            },
            Clear => {
                self.model.any = false;
                self.a.set_active(false);
                self.b.set_active(false);
                self.c.set_active(false);
                self.d.set_active(false);
                self.e.set_active(false);
                self.any.set_active(false);
            },
            SetLabel(text) => self.model.label = text,
            _ => {},
        }
    }

    view! {
        gtk::Grid {
            gtk::Label {
                text: self.model.label.as_str(),
                hexpand: true,
                halign: gtk::Align::Start,
                cell: {
                    left_attach: 0,
                    top_attach: 0,
                    width: 2,
                }
            },
            #[name="a"]
            gtk::CheckButton {  // A
                cell: {
                    left_attach: 2,
                    top_attach: 0
                },
                hexpand: true,
                label: "A",
                sensitive: !self.model.any,
                toggled(cbox) => CheckboxChanged(Checkbox::A, cbox.get_active()),
            },
            #[name="b"]
            gtk::CheckButton {  // B
                cell: {
                    left_attach: 3,
                    top_attach: 0
                },
                hexpand: true,
                label: "B",
                sensitive: !self.model.any,
                toggled(cbox) => CheckboxChanged(Checkbox::B, cbox.get_active()),
            },
            #[name="c"]
            gtk::CheckButton {  // C
                cell: {
                    left_attach: 4,
                    top_attach: 0
                },
                hexpand: true,
                label: "C",
                sensitive: !self.model.any,
                toggled(cbox) => CheckboxChanged(Checkbox::C, cbox.get_active()),
            },
            #[name="d"]
            gtk::CheckButton {  // D
                cell: {
                    left_attach: 5,
                    top_attach: 0
                },
                hexpand: true,
                label: "D",
                sensitive: !self.model.any,
                toggled(cbox) => CheckboxChanged(Checkbox::D, cbox.get_active()),
            },
            #[name="e"]
            gtk::CheckButton {  // E
                cell: {
                    left_attach: 6,
                    top_attach: 0
                },
                hexpand: true,
                label: "E",
                sensitive: !self.model.any,
                toggled(cbox) => CheckboxChanged(Checkbox::E, cbox.get_active()),
            },
            #[name="any"]
            gtk::CheckButton {  // Any
                cell: {
                    left_attach: 7,
                    top_attach: 0
                },
                hexpand: true,
                label: "Any",
                toggled(cbox) => CheckboxChanged(Checkbox::Any, cbox.get_active()),
            }
        }
    }
}


#[widget]
impl Widget for QuestionGrid {
    fn model() -> QuestionGridModel {
        let mut checked = HashMap::new();
        for i in 1..6 {
            checked.insert((i, Checkbox::A), false);
            checked.insert((i, Checkbox::B), false);
            checked.insert((i, Checkbox::C), false);
            checked.insert((i, Checkbox::D), false);
            checked.insert((i, Checkbox::E), false);
            checked.insert((i, Checkbox::Any), false);
        }

        QuestionGridModel {
            checked: checked,
            answers: Vec::new(),
        }
    }

    fn update(&mut self, event: QuestionMsg) {
        match event {
            Clear => {
                self.model = QuestionGrid::blank_model();
                self.q1.stream().emit(Clear);
                self.q2.stream().emit(Clear);
                self.q3.stream().emit(Clear);
                self.q4.stream().emit(Clear);
                self.q5.stream().emit(Clear);
            }
            UpdateChecked(num, cbox, state) => {
                self.model.checked.insert((num, cbox), state);
                self.update_answers();
            },
            Submit => {
                // nothing for now
            }
            _ => {},
        }
    }

    fn update_answers(&mut self) {
        let mut answers: Vec<Vec<AnswerChoice>> = Vec::new();
        let all_answers: Vec<AnswerChoice> = vec![
            AnswerChoice::A,
            AnswerChoice::B,
            AnswerChoice::C,
            AnswerChoice::D,
            AnswerChoice::E,
        ];
        for i in 1..6 {
            if let &true = self.model.checked.get(&(i, Checkbox::Any)).unwrap() {
                answers.push(all_answers.clone());
                continue;
            } else {
                let mut selected: Vec<AnswerChoice> = Vec::new();
                if let &true = self.model.checked.get(&(i, Checkbox::A)).unwrap() {
                    selected.push(AnswerChoice::A);
                }
                if let &true = self.model.checked.get(&(i, Checkbox::B)).unwrap() {
                    selected.push(AnswerChoice::B);
                }
                if let &true = self.model.checked.get(&(i, Checkbox::C)).unwrap() {
                    selected.push(AnswerChoice::C);
                }
                if let &true = self.model.checked.get(&(i, Checkbox::D)).unwrap() {
                    selected.push(AnswerChoice::D);
                }
                if let &true = self.model.checked.get(&(i, Checkbox::E)).unwrap() {
                    selected.push(AnswerChoice::E);
                }
                answers.push(selected);
            }
        }
        self.model.answers = answers;
    }

    fn blank_model() -> QuestionGridModel {
        let mut checked = HashMap::new();
        for i in 1..6 {
            checked.insert((i, Checkbox::A), false);
            checked.insert((i, Checkbox::B), false);
            checked.insert((i, Checkbox::C), false);
            checked.insert((i, Checkbox::D), false);
            checked.insert((i, Checkbox::E), false);
            checked.insert((i, Checkbox::Any), false);
        }

        QuestionGridModel {
            checked: checked,
            answers: Vec::new(),
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Vertical,
            #[name="q1"]
            Question {
                SetLabel: String::from("Question 1"),
                CheckboxChanged(ref choice, state) => UpdateChecked(1, choice.clone(), state),
            },
            #[name="q2"]
            Question {
                SetLabel: String::from("Question 2"),
                CheckboxChanged(ref choice, state) => UpdateChecked(2, choice.clone(), state),
            },
            #[name="q3"]
            Question {
                SetLabel: String::from("Question 3"),
                CheckboxChanged(ref choice, state) => UpdateChecked(3, choice.clone(), state),
            },
            #[name="q4"]
            Question {
                SetLabel: String::from("Question 4"),
                CheckboxChanged(ref choice, state) => UpdateChecked(4, choice.clone(), state),
            },
            #[name="q5"]
            Question {
                SetLabel: String::from("Question 5"),
                CheckboxChanged(ref choice, state) => UpdateChecked(5, choice.clone(), state),
            },
            gtk::Box {
                orientation: Orientation::Horizontal,
                margin_top: 5,
                gtk::Button {
                    label: "Clear",
                    hexpand: true,
                    halign: gtk::Align::End,
                    margin_right: 10,
                    clicked => Clear,
                },
                gtk::Button {
                    label: "Submit",
                    halign: gtk::Align::End,
                    clicked => Submit,
                }
            }
        }
    }
}


#[widget]
impl Widget for Application {
    fn model() -> ClickerModel {
        ClickerModel {
            // empty for now
        }
    }

    fn update(&mut self, event: ClickerMsg) {
        match event {
            ClickerMsg::Quit => gtk::main_quit(),
            _ => {}
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Orientation::Vertical,
                margin_left: 5,
                margin_right: 5,
                margin_top: 10,
                margin_bottom: 10,
                QuestionGrid {
                    // empty for now
                }
            },
            delete_event(_, _) => (ClickerMsg::Quit, Inhibit(false)),
        }
    }
}
