use std::io;

use crossterm::event::{self, Event, KeyEvent, KeyModifiers, /*KeyCode,*/ KeyEventKind};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use tui_prompts::prelude::*;

mod tui;

//Field holds the current focused field
#[derive(Debug, Default)]
enum Field {
    #[default]
    Server,
    Username,
    Password,
}

//App holds all fields + some other stuff
#[derive(Debug, Default)]
pub struct App<'a> {
    exit: bool,
    
    current_field:   Field,
    server_state: TextState<'a>,
    username_state:  TextState<'a>,
    password_state:  TextState<'a>,
}

//implementation of App
impl <'a>App<'a> {
    /// runs the application's main loop until the user quits
    
    //main drawing loop
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        *self.current_state().focus_state_mut() = FocusState::Focused;
        while !self.exit {
            terminal.draw(|frame| self.draw_ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    //event handling loop
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    //handling all KeyEvents
    fn handle_key_event(&mut self, key_event: KeyEvent) {

        match (key_event.code, key_event.modifiers) {

            (event::KeyCode::Char('c'), KeyModifiers::CONTROL)=> self.exit(),
            (event::KeyCode::Enter, _) => self.submit(),
            (event::KeyCode::Tab, KeyModifiers::NONE) => self.focus_next(),
            (event::KeyCode::BackTab, KeyModifiers::SHIFT) => self.focus_prev(),
            _ => self.focus_handle_event(key_event),
        }

    }
    
    //kill the program on q
    fn exit(&mut self) {
        self.exit = true;
    }

    //Main UI drawing function, called in run() 
    fn draw_ui(&mut self, frame: &mut Frame) {

        let (username_area, password_area, server_area, value_area, _debug_area) = self.arrange_login(frame.size());

        let title = Title::from(" Secure Virtual Pets ".bold());
        let quit_instruction = Title::from(" Ctrl C to quit ");
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(quit_instruction.alignment(Alignment::Center).position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);


        frame.render_widget(block, frame.size());

        self.draw_text_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_server_prompt(frame, server_area);
        self.draw_state_value(frame, value_area);
        //self.draw_debug(frame, debug_area);    //keep commented unless you just want to see the state of the program

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
    fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Server => &mut self.server_state,
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
        }
    }
    
    // draw the value of the current state underneath the prompts for debugging purposes
    fn draw_state_value(&mut self, frame: &mut Frame, value_area: Rect) {
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
    fn draw_text_prompt(&mut self, frame: &mut Frame, username_area: Rect) {
        TextPrompt::from("Username").draw(frame, username_area, &mut self.username_state);
    }

    fn draw_password_prompt(&mut self, frame: &mut Frame, password_area: Rect) {
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, password_area, &mut self.password_state);
    }

    fn draw_server_prompt(&mut self, frame: &mut Frame, server_area: Rect) {
        TextPrompt::from("Server").draw(frame, server_area, &mut self.server_state);
    }


    //focusing between states
    fn focus_handle_event(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }

    fn focus_next(&mut self) {
        self.current_state().blur();
        self.current_field = self.next_field();
        self.current_state().focus();
    }

    fn focus_prev(&mut self) {
        self.current_state().blur();
        self.current_field = self.prev_field();
        self.current_state().focus();
    }
    
    //swtiching between states
    fn next_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Server,
            Field::Server => Field::Username,
        }
    }

    fn prev_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Server,
            Field::Password => Field::Username,
            Field::Server => Field::Password,
        }
    }
    
    //submitting a string to a state
    fn submit(&mut self) {
        self.current_state().complete();
        if self.current_state().is_finished() && !self.is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        }
    }

    //when all states have been finished
    fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.server_state.is_finished()
    }


}

//testing basic functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

        Ok(())
    }

}

//main function
fn main() -> io::Result<()> {

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result

}


