use crate::App;
use crate::Field;

use crossterm::event::KeyEvent;

use ratatui::prelude::*;

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
        self.draw_debug(frame, _debug_area);

    }

    //arrange the areas of the text prompts
    fn arrange_login(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = Rect::new(88, 20, area.width, 1);
        let username_area  = Rect::new(88, 21, area.width, 1);
        let password_area = Rect::new(88, 22, area.width, 1);
        let value_area     = Rect::new(88, 23, area.width, 1);
        let debug_area     = Rect::new(88, 24, area.width, 20);

        (username_area, password_area, server_area, value_area, debug_area)
    }
    
    //swtiching between states
    //I could differnetiate this by screen like on the prompts, but would that really be clearer?
    pub fn next_login_field(&mut self) -> Field {
        match self.current_field {
            Field::Email => Field::Server, //this should never be hit
            Field::Username => Field::Password,
            Field::Password => Field::Server,
            Field::Server => Field::Username,
        }
    }

    pub fn prev_login_field(&mut self) -> Field {
        match self.current_field {
            Field::Email => Field::Server, //this should never be hit
            Field::Username => Field::Server,
            Field::Password => Field::Username,
            Field::Server => Field::Password,
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
            self.current_field = self.next_login_field();
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
