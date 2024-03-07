
use crate::App;
use crate::Field;


use crossterm::event::KeyEvent;

use ratatui:: prelude::*;

use tui_prompts::prelude::*;

//This is the implementation for the signup screen
impl <'a> App <'a> {

    pub fn draw_signup_screen(&mut self, frame: &mut Frame) {

        let (server_area, email_area, username_area, password_area, value_area) = self.arrange_signup(frame.size());
        
        //and draw eact of the prompts
        self.draw_server_prompt(frame, server_area);
        self.draw_email_prompt(frame, email_area);
        self.draw_username_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_state_value(frame, value_area);

    }

    fn arrange_signup(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = Rect::new(88, 20, area.width, 1);
        let email_area     = Rect::new(88, 21, area.width, 1);
        let username_area  = Rect::new(88, 22, area.width, 1);
        let password_area = Rect::new(88, 23, area.width, 1);
        let value_area     = Rect::new(88, 24, area.width, 1);

        ( server_area, email_area, username_area, password_area, value_area)
    }


    pub fn focus_next_signup_prompt(&mut self) {
        self.current_signup_state().blur();
        self.current_field = self.next_signup_field();
        self.current_signup_state().focus();
    }

    pub fn focus_prev_signup_prompt(&mut self) {
        self.current_signup_state().blur();
        self.current_field = self.prev_signup_field();
        self.current_signup_state().focus();
    }

    //swtiching between states
    fn next_signup_field(&mut self) -> Field {
        match self.current_field {
            Field::Email => Field::Username,
            Field::Server => Field::Email,
            Field::Username => Field::Password,
            Field::Password => Field::Server,
        }
    }

    fn prev_signup_field(&mut self) -> Field {
        match self.current_field {
            Field::Email => Field::Server, //this should never be hit
            Field::Server => Field::Password,
            Field::Username => Field::Email,
            Field::Password => Field::Username,
        }
    }

    fn draw_email_prompt(&mut self, frame: &mut Frame, email_area: Rect) {
        TextPrompt::from("Email").draw(frame, email_area, &mut self.email_state);
    }

    pub fn signup_submit(&mut self) {
        self.current_signup_state().complete();
        if self.current_signup_state().is_finished() && !self.is_finished() {
            self.current_signup_state().blur();
            self.current_field = self.next_signup_field();
            self.current_signup_state().focus();
        }
    }

    fn current_signup_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Email => &mut self.email_state,
            Field::Server => &mut self.server_state,
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
        }
    }


    pub fn focus_signup_handle_event(&mut self, key_event: KeyEvent) {
        let state = self.current_signup_state();
        state.handle_key_event(key_event);
    }

}
