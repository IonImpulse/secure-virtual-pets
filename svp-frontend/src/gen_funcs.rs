use crate::App;
use crate::Field;
use crate::Screen;

use crossterm::event::KeyEvent;

use ratatui::{
    prelude::*,
    widgets::{*},
};

use tui_prompts::prelude::*;


//This implementation contains 
impl <'a> App <'a> {

    // ================================= Drawing the prompts =================================
    pub fn draw_username_prompt(&mut self, frame: &mut Frame, username_area: Rect) {
        TextPrompt::from("Username").draw(frame, username_area, &mut self.username_state);
    }

    pub fn draw_password_prompt(&mut self, frame: &mut Frame, password_area: Rect) {
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, password_area, &mut self.password_state);
    }

    pub fn draw_server_prompt(&mut self, frame: &mut Frame, server_area: Rect) {
        TextPrompt::from("Server").draw(frame, server_area, &mut self.server_state);
    }

    pub fn draw_email_prompt(&mut self, frame: &mut Frame, email_area: Rect) {
        TextPrompt::from("Email").draw(frame, email_area, &mut self.email_state);
    }

    // draw the value of the current state underneath the prompts for debugging purposes
    pub fn draw_state_value(&mut self, frame: &mut Frame, value_area: Rect) {
        let state = self.current_state();
        let state = format!("  Value: {}", state.value());
        frame.render_widget(
            Paragraph::new(state).style(Style::new().dark_gray()),
            value_area,
        );
    }
    // draw a debug string in the top right corner of the screen that shows the current state of
    // the app.
    pub fn draw_debug(&mut self, frame: &mut Frame, area: Rect) {
        if false {
            return;
        }
        let debug = format!("{self:#?}");
        frame.render_widget(
            Paragraph::new(debug)
            .wrap(Wrap { trim: false })
            .block(Block::new().borders(Borders::LEFT)),
            area,
            );
    }


    // ================================= State matching and Focusing =================================
    pub fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Email => &mut self.email_state, //should never be hit
            Field::Server => &mut self.server_state,
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
        }
    }

    pub fn focus_next_prompt(&mut self) {
        self.current_state().blur();
        self.current_field = self.next_field();
        self.current_state().focus();
    }

    pub fn focus_prev_prompt(&mut self) {
        self.current_state().blur();
        self.current_field = self.prev_field();
        self.current_state().focus();
    }

    //Next field using a nested match function to deterimine which field to actually focus too
    //based on the currently displayed screen
    pub fn next_field(&mut self) -> Field {

        match self.current_screen {

            Screen::Login => match self.current_field {
                Field::Email => Field::Server, //this should never be hit
                Field::Username => Field::Password,
                Field::Password => Field::Server,
                Field::Server => Field::Username,
            }
            Screen::Signup => match self.current_field {
                Field::Email => Field::Username,
                Field::Server => Field::Email,
                Field::Username => Field::Password,
                Field::Password => Field::Server,

            }
        }
    }

    //Prev field using a nested match function to deterimine which field to actually focus too
    //based on the currently displayed screen
    pub fn prev_field(&mut self) -> Field {

        match self.current_screen {

            Screen::Login => match self.current_field {
                Field::Email => Field::Server, //this should never be hit
                Field::Username => Field::Server,
                Field::Password => Field::Username,
                Field::Server => Field::Password,
            }
            Screen::Signup => match self.current_field {
                Field::Email => Field::Server, //this should never be hit
                Field::Server => Field::Password,
                Field::Username => Field::Email,
                Field::Password => Field::Username,

            }
        }
    }

    //focusing between states
    pub fn focus_handle_event(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }


    //submitting a string to a state
    pub fn submit(&mut self) {
        self.current_state().complete();
        if self.current_state().is_finished() && !self.is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        }
    }


    //when all (login) states have been finished
    //TODO: Figure out what exactly this does and how it can be used to trigger the submisson of
    //strings
    pub fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.server_state.is_finished()
    }

}

