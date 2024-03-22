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
        //self.current_state().();
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

    pub fn dynamic_rect(&self, frame_size: Rect, x_coordinate: u16, y_coordinate: u16, x_size: u16, y_size: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                         //split the screen into three vertical slices, leaving the middle most
                         //slice as the perecent of the area specificed 
                         Constraint::Percentage(100 - ((100 - ((y_size * y_coordinate)) + y_size))),
                         Constraint::Percentage(y_size),
                         Constraint::Percentage(100 - (y_size * y_coordinate)),
            ])
            .split(frame_size);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                         Constraint::Percentage(100 - ((100 - ((x_size * x_coordinate)) + x_size))),
                         Constraint::Percentage(x_size),
                         Constraint::Percentage(100 - (x_size * x_coordinate)),
            ])
            //And then split the middle most slice horizaontally three times, and return the middle
            //one
            .split(popup_layout[1])[1]
    }

}

