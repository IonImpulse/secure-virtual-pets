use crate::App;

use ratatui::{
    prelude::*,
    widgets::{*},
};

//use crate::Borders;
//use crate::Block;

//implementation for the Login screen for App
impl <'a> App <'a> {
    pub fn draw_login_screen(&mut self, frame: &mut Frame) {

        let (username_area, password_area, server_area, value_area, _debug_area) = self.arrange_login(frame.size());

        //and draw eact of the prompts
        //frame.render_widget(Block::default().borders(Borders::all()).title("Main"), self.dynamic_rect(frame.size(), 4, 5, 16, 10));
        self.draw_username_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_server_prompt(frame, server_area);
        //self.draw_server_table(frame, server_area);
        self.draw_state_value(frame, value_area);
        //self.draw_debug(frame, _debug_area);

    }

    //arrange the areas of the text prompts
    fn arrange_login(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let server_area  = self.dynamic_rect(area, 4, 10, 16, 2); //this is the area for the "text prompt version of the server
        //let server_area  = self.dynamic_rect(area, 4, 5, 16, 10); //This is the temporary table one
        let username_area  = self.dynamic_rect(area, 4, 11, 16, 2);
        let password_area = self.dynamic_rect(area, 4, 12, 16, 2);
        let value_area     = self.dynamic_rect(area, 4, 13, 16, 2);
        let debug_area     = self.dynamic_rect(area, 4, 14, 16, 2);

        (username_area, password_area, server_area, value_area, debug_area)
    }

    // login is the only screen that draws a server table so we define it's draw function here
    // Actually I'm going to defer this to another time, basic functionality like connecting to the
    // backend needs to be implemented first
    pub fn draw_server_table(&mut self, frame: &mut Frame, server_area: Rect) {

        let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(10),
        ];
        let table = Table::new(rows, widths)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // You can set the style of the entire Table.
            .style(Style::new().blue())
            // It has an optional header, which is simply a Row always visible at the top.
            .header(
                Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::new().bold())
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
                )
            // It has an optional footer, which is simply a Row always visible at the bottom.
            .footer(Row::new(vec!["Updated on Dec 28"]))
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::default().title("Table"))
            // The selected row and its content can also be styled.
            .highlight_style(Style::new().reversed())
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(">>");
        frame.render_stateful_widget(table, server_area, &mut self.table_state);
    }
}
