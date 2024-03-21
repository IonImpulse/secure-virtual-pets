use crate::App;

use ratatui::prelude::*;

//implementation for the Login screen for App
impl <'a> App <'a> {
    pub fn draw_login_screen(&mut self, frame: &mut Frame) {

        let (username_area, password_area, server_area, value_area, _debug_area) = self.arrange_login(frame.size());

        //and draw eact of the prompts
        //frame.render_widget(Block::default().borders(Borders::all()).title("Main"), self.dynamic_rect(frame.size(), 4, 20, 16, 1));
        self.draw_username_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_server_prompt(frame, server_area);
        self.draw_state_value(frame, value_area);
        //self.draw_debug(frame, _debug_area);

    }

    //arrange the areas of the text prompts
    fn arrange_login(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = self.dynamic_rect(area, 4, 10, 16, 2);
        let username_area  = self.dynamic_rect(area, 4, 11, 16, 2);
        let password_area = self.dynamic_rect(area, 4, 12, 16, 2);
        let value_area     = self.dynamic_rect(area, 4, 13, 16, 2);
        let debug_area     = self.dynamic_rect(area, 4, 14, 16, 2);

        (username_area, password_area, server_area, value_area, debug_area)
    }
}
