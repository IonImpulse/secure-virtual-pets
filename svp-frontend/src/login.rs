use crate::App;


use ratatui::prelude::*;

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
        let server_area  = self.centered_rect(Rect::new(88, 20, area.width, 1), 50, 50);
        let username_area  = Rect::new(88, 21, area.width, 1);
        let password_area = Rect::new(88, 22, area.width, 1);
        let value_area     = Rect::new(88, 23, area.width, 1);
        let debug_area     = Rect::new(88, 24, area.width, 20);

        (username_area, password_area, server_area, value_area, debug_area)
    }
}
