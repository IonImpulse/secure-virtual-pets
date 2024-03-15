use crate::App;

use ratatui:: prelude::*;

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
}
