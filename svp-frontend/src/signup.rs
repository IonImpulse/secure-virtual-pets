
use crate::App;
use crate::Field;

use crossterm::event::KeyEvent;

use ratatui::{
    prelude::*,
    widgets::{*},
};

use tui_prompts::prelude::*;

//This is the implementation for the signup screen
impl <'a> App <'a> {

    fn draw_signup_screen(&mut self, frame: &mut Frame) {

        let (email_area, username_area, password_area, value_area) = self.arrange_signup(frame.size());
        
        //define block and titles
        let title = Title::from(" Secure Virtual Pets ".bold());
        let quit_instruction = Title::from(" Ctrl C to quit ");
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(quit_instruction.alignment(Alignment::Center).position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        //draw the block
        frame.render_widget(block, frame.size());
        
        //and draw eact of the prompts
        self.draw_server_prompt(frame, email_area);
        self.draw_text_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_state_value(frame, value_area);

    }

    fn arrange_signup(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = Rect::new(88, 20, area.width, 1);
        let username_area  = Rect::new(88, 21, area.width, 1);
        let password_area = Rect::new(88, 22, area.width, 1);
        let value_area     = Rect::new(88, 23, area.width, 1);
        let debug_area     = Rect::new(88, 24, area.width, 1);

        (username_area, password_area, server_area, value_area, debug_area)
    }

    
}
