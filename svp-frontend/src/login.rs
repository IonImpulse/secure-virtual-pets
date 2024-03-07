///This file contains the implementaition for drawing and managing the login screen

use crate::App;
use crate::Field;

use crossterm::event::KeyEvent;

use ratatui::{
    prelude::*,
    widgets::{*},
};

use tui_prompts::prelude::*;

//implementation for the Login screen for App
impl <'a> App <'a> {
    pub fn draw_login_screen(&mut self, frame: &mut Frame) {

        let (username_area, password_area, server_area, value_area, _debug_area) = self.arrange_login(frame.size());
        //and draw eact of the prompts
        self.draw_username_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_server_prompt(frame, server_area);
        self.draw_state_value(frame, value_area);

    }

    //arrange the areas of the text prompts
    fn arrange_login(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = Rect::new(88, 20, area.width, 1);
        let username_area  = Rect::new(88, 21, area.width, 1);
        let password_area = Rect::new(88, 22, area.width, 1);
        let value_area     = Rect::new(88, 23, area.width, 1);
        let debug_area     = Rect::new(88, 24, area.width, 1);

        (username_area, password_area, server_area, value_area, debug_area)
    }

    //matching the current state to the app state
    pub fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Server => &mut self.server_state,
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
        }
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
    // commented out to avoid the warning
    //fn draw_debug(&mut self, frame: &mut Frame, area: Rect) {
        //if false {
            //return;
        //}
        //let debug = format!("{self:#?}");
        //frame.render_widget(
            //Paragraph::new(debug)
                //.wrap(Wrap { trim: false })
                //.block(Block::new().borders(Borders::LEFT)),
            //area,
        //);
    //}

    // Drawing the prompts
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


    //focusing between states
    pub fn focus_handle_event(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }

    pub fn focus_next_login_prompt(&mut self) {
        self.current_state().blur();
        self.current_field = self.next_field();
        self.current_state().focus();
    }

    pub fn focus_prev_login_prompt(&mut self) {
        self.current_state().blur();
        self.current_field = self.prev_field();
        self.current_state().focus();
    }
    
    //swtiching between states
    pub fn next_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Server,
            Field::Server => Field::Username,
        }
    }

    pub fn prev_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Server,
            Field::Password => Field::Username,
            Field::Server => Field::Password,
        }
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

    //when all states have been finished
    pub fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.server_state.is_finished()
    }
}
