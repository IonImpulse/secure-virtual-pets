use std::io;

//TODO: "Unfinish" states when moving beteween screens. Need to read up on tui_prompts to implement
//that

use crossterm::event::{self, Event, KeyEvent, KeyModifiers, /*KeyCode,*/ KeyEventKind};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use tui_prompts::prelude::*;

mod gen_funcs;
mod tui;
mod login;
mod signup;


//Screen holds the current focused Screen (for determining what keyevents do what)
#[derive(Debug, Default, PartialEq)]
pub enum Screen {
    #[default]
    Login,
    Signup,
}

//Field holds the current focused field
#[derive(Debug, Default)]
pub enum Field {
    #[default]
    Server,
    Username,
    Password,
    Email,
}

//App holds all fields + some other stuff
#[derive(Debug, Default)]
pub struct App<'a> {
    exit: bool,
    current_screen: Screen, 
    current_field:   Field,

    server_state: TextState<'a>,
    email_state: TextState<'a>,
    username_state:  TextState<'a>,
    password_state:  TextState<'a>,
}

//main implementation block of App
//holds main loop, event handling, etc
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

        match (key_event.code, key_event.modifiers, &self.current_screen) {
            
            //swtich screens
            (event::KeyCode::Char('s'), KeyModifiers::CONTROL, _) => self.switch_screen(),

            //submit to a field 
            (event::KeyCode::Enter, _, _ ) => self.submit(),
            
            //focus between prompts (determined conditionally based on what screen is focused) 
            (event::KeyCode::Tab, KeyModifiers::NONE, _ ) => self.focus_next_prompt(),
            (event::KeyCode::BackTab, KeyModifiers::SHIFT, _) => self.focus_prev_prompt(),

            //kill the frontend
            (event::KeyCode::Char('c'), KeyModifiers::CONTROL, _ )=> self.exit(),

            (_,_,_) => self.focus_handle_event(key_event),
            

        }
    }
    

    //Main UI drawing function, called in run() 
    fn draw_ui(&mut self, frame: &mut Frame) {

        //I'm thinking for the moment, the framing block will be a constant in the TUI, so I'll
        //include it in the base draw function. 
        
        //define block and titles
        let title = Title::from(" Secure Virtual Pets ".bold());
        let quit_instruction = Title::from(" Ctrl C to quit - Ctrl S to Switch Screens ");
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(quit_instruction.alignment(Alignment::Center).position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        //draw the block
        frame.render_widget(block, frame.size()); 

        if self.current_screen == Screen::Login{
            self.draw_login_screen(frame);
        } else {
        self.draw_signup_screen(frame);
        }

    }

    //kill the program on cntrl c 
    fn exit(&mut self) {
        self.exit = true;
    }
    
    //switches the screen
    fn switch_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Login => Screen::Signup,
            Screen::Signup => Screen::Login,
        };
    }
}


//main function
fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result

}


