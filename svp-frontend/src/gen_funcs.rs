use crate::App;
use crate::Field;
use crate::Screen;

use ratatui::{
    prelude::*,
    widgets::{*},
};

use tui_prompts::prelude::*;

impl <'a> App <'a> {
    // draw the value of the current state underneath the prompts for debugging purposes
    pub fn draw_state_value(&mut self, frame: &mut Frame, value_area: Rect) {
        let state = self.current_state();
        let state = format!("  Value: {}", state.value());
        frame.render_widget(
            Paragraph::new(state).style(Style::new().dark_gray()),
            value_area,
        );
    }

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

    pub fn draw_email_prompt(&mut self, frame: &mut Frame, email_area: Rect) {
        TextPrompt::from("Email").draw(frame, email_area, &mut self.email_state);
    }

    //matching the current state to the app state
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

        match self.current_screen { 
        Screen::Login => self.current_field = self.next_login_field(),
        Screen::Signup => self.current_field = self.next_signup_field(),
        }

        self.current_state().focus();
    }

    pub fn focus_prev_prompt(&mut self) {
        self.current_state().blur();
        match self.current_screen { 
        Screen::Login => self.current_field = self.prev_login_field(),
        Screen::Signup => self.current_field = self.prev_signup_field(),
        }
        self.current_state().focus();
    }

    // draw a debug string in the top right corner of the screen that shows the current state of
    // the app.
    // commented out to avoid the warning
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



}

