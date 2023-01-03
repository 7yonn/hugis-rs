use crate::window;
use window::*;
use std::io;
use std::fmt;
use std::error;
use crate::parser::Commands::*;

#[derive(Debug, Clone, Copy)]
pub enum Commands {
    Draw,
    Clear,
    New(isize, isize),
    Resize(isize, isize),
    Print,
    Fill(char),
    Replace(char, char),
    Help,
    Quit,
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidCommandError(String),
    TooManyArguments(String),
    MissingArguments(String),
    NotNumber(String),
    NoNonPositiveIntegers(String),
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidCommandError(input) => {
                write!(f, "\"{input}\" is not a valid command.!")
            },
            ParseError::MissingArguments(input) => {
                write!(f, "\"{input}\", missing arguments!")
            },
            ParseError::TooManyArguments(input) => {
                write!(f, "\"{input}\", too many arguments!")
            },
            ParseError::NotNumber(input) => {
                write!(f, "\"{input}\", not a number!")
            },
            ParseError::NoNonPositiveIntegers(input) => {
                write!(f, "\"{input}\", can't have numbers below 1!")
            }
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::InvalidCommandError(_) => None,
            _ => None,
        }
    }
}


pub fn parse_to_command(input: String) -> Result<Commands>{
    match input.split_once(' ') {
        None => {
            match input.as_str() {
                "print" => Ok(Print),
                "quit" => Ok(Quit),
                "help" => Ok(Help),
                "clear" => Ok(Clear),
                _ => Err(ParseError::InvalidCommandError(input))
            }
        }
        Some((command, args)) => {
            match command {
                "fill" => {
                    let chars: Vec<char> = args.chars().collect();
                    if chars.len() > 1 {
                        return Err(ParseError::TooManyArguments(args.to_owned()));
                    }
                    Ok(Fill(chars[0]))
                },
                "replace" => {
                    let args = args.split_once(' ');
                    if let Some((arg1, arg2)) = args {
                        if arg1.len() > 1 || arg2.len() > 1 {
                            return Err(ParseError::TooManyArguments(input));
                        }
                        return Ok(Replace(arg1.chars().nth(0).unwrap(), arg2.chars().nth(0).unwrap()))
                    } else {
                        return Err(ParseError::MissingArguments(input));
                    }
                },
                "new" => {
                    let arg_vec: Vec<_> = args.split(' ').collect();
                    if arg_vec.len() > 2 {
                        return Err(ParseError::TooManyArguments(input));
                    }
                    let arg1 = arg_vec[0].parse::<isize>();
                    let arg2 = arg_vec[1].parse::<isize>();
                    let arg1= if let Ok(arg1) = arg1 { arg1 } else {
                        return Err(ParseError::NotNumber(args.to_owned()));
                    };
                    let arg2= if let Ok(arg2) = arg2 { arg2 } else {
                        return Err(ParseError::NotNumber(args.to_owned()));
                    };
                    if arg1 <= 0 || arg2 <= 0 {
                        return Err(ParseError::NoNonPositiveIntegers(args.to_owned()));
                    }
                    return Ok(New(arg1, arg2))
                },
                "resize" => {
                    let arg_vec: Vec<_> = args.split(' ').collect();
                    if arg_vec.len() > 2 {
                        return Err(ParseError::TooManyArguments(input));
                    }
                    let arg1 = arg_vec[0].parse::<isize>();
                    let arg2 = arg_vec[1].parse::<isize>();
                    let arg1= if let Ok(arg1) = arg1 { arg1 } else {
                        return Err(ParseError::NotNumber(args.to_owned()));
                    };
                    let arg2= if let Ok(arg2) = arg2 { arg2 } else {
                        return Err(ParseError::NotNumber(args.to_owned()));
                    };
                    if arg1 <= 0 || arg2 <= 0 {
                        return Err(ParseError::NoNonPositiveIntegers(args.to_owned()));
                    }
                    return Ok(Resize(arg1, arg2))
                },
                _ => Err(ParseError::InvalidCommandError(input))
            }
        }
    }
}

pub fn command_from_input() -> Result<Commands> {
    let input = get_input().unwrap();
    parse_to_command(input)
}

pub fn get_input() -> io::Result<String> {
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
