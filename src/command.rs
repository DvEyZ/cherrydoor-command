use std::{fmt::Display, error::Error};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Command {
    open :Option<bool>,
    close :Option<bool>,
    open_for :Option<i32>,

    display_text :Option<String>,
    display_text_for :Option<(String, i32)>,

    set_color :Option<(u8, u8, u8)>,
    set_color_for :Option<(u8, u8, u8, i32)>,

    play_sound :Option<i32>,

    backlight_on :Option<bool>,
    backlight_off :Option<bool>
}

#[derive(Debug)]
pub struct CommandBuildError {
    message :String
}

impl Error for CommandBuildError {}

impl Display for CommandBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Command {
    pub fn new() -> Self {
        Command {
            open: None,
            close: None,
            open_for: None,
            display_text: None,
            display_text_for: None,
            set_color: None,
            set_color_for: None,
            play_sound :None,
            backlight_on :None,
            backlight_off :None
        }
    }

    pub fn open(mut self) -> Self {
        self.open = Some(true);
        self
    }

    pub fn close(mut self) -> Self {
        self.close = Some(true);
        self
    }

    pub fn open_for(mut self, time :i32) -> Self {
        self.open_for = Some(time);
        self
    }

    pub fn display_text(mut self, text :String) -> Self {
        self.display_text = Some(text);
        self
    }

    pub fn display_text_for(mut self, text :String, time :i32) -> Self {
        self.display_text_for = Some((text, time));
        self
    }

    pub fn set_color(mut self, r :u8, g :u8, b :u8) -> Self {
        self.set_color = Some((r, g ,b));
        self
    }

    pub fn set_color_for(mut self, r :u8, g :u8, b :u8, time :i32) -> Self {
        self.set_color_for = Some((r, g, b, time));
        self
    }

    pub fn play_sound(mut self, sound_id :i32) -> Self {
        self.play_sound = Some(sound_id);
        self
    }

    pub fn backlight_on(mut self) -> Self {
        self.backlight_on = Some(true);
        self
    }

    pub fn backlight_off(mut self) -> Self {
        self.backlight_off = Some(true);
        self
    }

    pub fn into_string(self) -> Result<String, CommandBuildError> {
        let mut command = 0;
        let mut args :Vec<String> = vec![];

        let open_ex = vec![self.open.is_some(), self.close.is_some(), self.open_for.is_some()];
        let display_ex =  vec![self.display_text.is_some(), self.display_text_for.is_some()];
        let set_color_ex = vec![self.set_color.is_some(), self.set_color_for.is_some()];
        let backlight_ex = vec![self.backlight_on.is_some(), self.backlight_on.is_some()];

        let ex = vec![open_ex, display_ex, set_color_ex, backlight_ex];

        match ex.into_iter().map(|v| {
            v.into_iter().filter(|v| *v).collect::<Vec<bool>>().len() > 1
        }).find(|v| *v) {
            Some(_) => return Err(CommandBuildError {
                message: "Command contains elements that cannot coexist.".to_string()
            }),
            None => ()
        };

        if let Some(open) = self.open {
            if open {
                command |= 0x200;
            }
        }
        
        if let Some(close) = self.close {
            if close {
                command |= 0x100
            }
        }

        if let Some(time) = self.open_for {
            command |= 0x80;
            args.push(format!("{}", time));
        }

        if let Some(text) = self.display_text {
            command |= 0x40;
            args.push(text)
        }

        if let Some((text, time)) = self.display_text_for {
            command |= 0x20;
            args.push(text);
            args.push(format!("{}", time));
        }

        if let Some((r, g, b)) = self.set_color {
            command |= 0x10;
            args.push(format!("#{:x}{:x}{:x}", r, g, b));
        }

        if let Some((r, g, b, time)) = self.set_color_for {
            command |= 0x8;
            args.push(format!("#{:x}{:x}{:x}", r, g, b));
            args.push(format!("{}", time));
        }

        if let Some(sound_id) = self.play_sound {
            command |= 0x4;
            args.push(format!("{}", sound_id));
        }

        if let Some(backlight_on) = self.backlight_on {
            if backlight_on == true {
                command |= 0x2;
            }
        }

        if let Some(backlight_off) = self.backlight_off {
            if backlight_off == true {
                command |= 0x1;
            }
        }

        Ok(format!("{};{}", command, args.into_iter().fold(String::new(), |acc :String, v| {
            format!("{}{};", acc, v)
        })))
    }
}