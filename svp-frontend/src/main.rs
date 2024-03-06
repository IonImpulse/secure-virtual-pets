// old main.rs for the counter functionality using the tui module

use std::io;

use crossterm::event::{self, Event, KeyEvent, KeyModifiers, KeyCode, KeyEventKind};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use tui_prompts::prelude::*;

mod tui;

#[derive(Debug, Default)]
enum Field {
    #[default]
    Username,
    Password,
    Invisible,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    counter: i32,
    exit: bool,
    
    current_field: Field,
    invisible_state:   TextState<'a>,
    username_state: TextState<'a>,
    password_state: TextState<'a>,
}

impl <'a>App<'a> {
    /// runs the application's main loop until the user quits

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        *self.current_state().focus_state_mut() = FocusState::Focused;
        while !self.exit {
            //terminal.draw(|frame| self.render_frame(frame))?;
            terminal.draw(|frame| self.draw_ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

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

    fn handle_key_event(&mut self, key_event: KeyEvent) {

        match (key_event.code, key_event.modifiers) {

            (event::KeyCode::Left , _)=> self.decrement_counter(),
            (event::KeyCode::Right , _)=> self.increment_counter(),
            (event::KeyCode::Char('q'), _)=> self.exit(),
            (event::KeyCode::Enter, _) => self.submit(),
            (event::KeyCode::Tab, KeyModifiers::NONE) => self.focus_next(),
            (event::KeyCode::BackTab, KeyModifiers::SHIFT) => self.focus_prev(),
            _ => self.focus_handle_event(key_event),
        }

    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }


    fn draw_ui(&mut self, frame: &mut Frame) {
        let (username_area, password_area, invisible_area, value_area, debug_area) =
            self.split_layout(frame.size());
        self.draw_text_prompt(frame, username_area);
        self.draw_password_prompt(frame, password_area);
        self.draw_invisible_prompt(frame, invisible_area);
        self.draw_state_value(frame, value_area);
        //self.draw_debug(frame, debug_area);
        //

    }

    fn split_layout(&self, area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let (prompt_area, debug_area) = if false {
            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 2); 2])
                .split(area);
            (areas[0], areas[1])
        } else {
            (area, area)
        };
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); 4])
            .split(prompt_area);
        (areas[0], areas[1], areas[2], areas[3], debug_area)
    }


    fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
            Field::Invisible => &mut self.invisible_state,
        }
    }


    fn draw_text_prompt(&mut self, frame: &mut Frame, username_area: Rect) {
        TextPrompt::from("Username").draw(frame, username_area, &mut self.username_state);
    }

    fn draw_password_prompt(&mut self, frame: &mut Frame, password_area: Rect) {
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, password_area, &mut self.password_state);
    }

    fn draw_invisible_prompt(&mut self, frame: &mut Frame, invisible_area: Rect) {
        TextPrompt::from("Invisible")
            .with_render_style(TextRenderStyle::Invisible)
            .draw(frame, invisible_area, &mut self.invisible_state);
    }

    /// draw the value of the current state underneath the prompts.
    fn draw_state_value(&mut self, frame: &mut Frame, value_area: Rect) {
        let state = self.current_state();
        let state = format!("  Value: {}", state.value());
        frame.render_widget(
            Paragraph::new(state).style(Style::new().dark_gray()),
            value_area,
        );
    }

    /// draw a debug string in the top right corner of the screen that shows the current state of
    /// the app.
    fn draw_debug(&mut self, frame: &mut Frame, area: Rect) {
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

    fn submit(&mut self) {
        self.current_state().complete();
        if self.current_state().is_finished() && !self.is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        }
    }

    fn next_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Invisible,
            Field::Invisible => Field::Username,
        }
    }

    fn prev_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Invisible,
            Field::Password => Field::Username,
            Field::Invisible => Field::Password,
        }
    }

    fn is_finished(&self) -> bool {
        self.username_state.is_finished()
            && self.password_state.is_finished()
            && self.invisible_state.is_finished()
    }


}

impl <'a>Widget for &App<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }


}

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

fn main() -> io::Result<()> {

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result

}


