use crate::window;
use window::*;
use std::io;
use std::fmt;
use std::error;
use crate::parser::Commands::*;

#[derive(Debug, Clone, Copy)]
pub enum Commands {
    Clear,
    Draw(usize, Point, char),
    Fill(char),
    Help,
    List,
    NewWin(isize, isize),
    NewShape(Shape),
    Print,
    Quit,
    ToDo,
    Replace(char, char),
    Resize(isize, isize),
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidCommandError(String),
    TooManyArguments(String),
    MissingArguments(String),
    NotNumber(String),
    NoNonPositiveIntegers(String),
    InvalidShapeType(String),
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
                write!(f, "\"{input}\", not a valid number!")
            },
            ParseError::NoNonPositiveIntegers(input) => {
                write!(f, "\"{input}\", can't have numbers below 1!")
            }
            ParseError::InvalidShapeType(input) => {
                write!(f, "\"{input}\" is not a valid shape type")
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
    let mut parts = input.split_whitespace();
    let command = get_next(parts.next(), &input)?;
    match command {
        "print" => Ok(Print),
        "quit" => Ok(Quit),
        "help" => Ok(Help),
        "clear" => Ok(Clear),
        "list" => Ok(List),
        "fill" => {
            let chars = get_next(parts.next(), &input)?;
            if chars.len() > 1 {
                return Err(ParseError::TooManyArguments(chars.to_string()));
            }
            check_empty(parts.next(), &input)?;
            Ok(Fill(chars.chars().nth(0).unwrap()))
        },
        "replace" => {
            let arg1 = get_next(parts.next(), &input)?;
            let arg2 = get_next(parts.next(), &input)?;
            if arg1.len() > 1 || arg2.len() > 1 {
                return Err(ParseError::TooManyArguments(input));
            }
            check_empty(parts.next(), &input)?;

            return Ok(Replace(arg1.chars().nth(0).unwrap(), arg2.chars().nth(0).unwrap()))
        },
        "new" => {
            let option = get_next(parts.next(), &input)?;
            match option {
                "window" => {
                    let arg1 = parse_next_isize(parts.next(), &input)?;
                    let arg2 = parse_next_isize(parts.next(), &input)?;
                    check_empty(parts.next(), &input)?;
                    if arg1 <= 0 || arg2 <= 0 {
                        return Err(ParseError::NoNonPositiveIntegers(input));
                    }
                    return Ok(NewWin(arg1, arg2))
                },
                "shape" => {
                    let shape_type = parts.next().unwrap();
                    match shape_type {
                        "circle" => {
                            let radius = parse_next_isize(parts.next(), &input)?;
                            return Ok(NewShape(Shape::Circle(radius)));
                        },
                        "square" => {
                            let length = parse_next_isize(parts.next(), &input)?;
                            let height = parse_next_isize(parts.next(), &input)?;
                            check_empty(parts.next(), &input)?;
                            return Ok(NewShape(Shape::Square(length,height)));
                        },
                        _ => {return Err(ParseError::InvalidShapeType(shape_type.to_string()));},
                    }
                },
                _ => {
                    return Err(ParseError::InvalidCommandError(input));
                }
            }
        },
        "resize" => {
            let arg1 = parse_next_isize(parts.next(), &input)?;
            let arg2 = parse_next_isize(parts.next(), &input)?;
            check_empty(parts.next(), &input)?;
            if arg1 <= 0 || arg2 <= 0 {
                return Err(ParseError::NoNonPositiveIntegers(input));
            }
            return Ok(Resize(arg1, arg2))
        },
        "draw" => {
            let index = parse_next_usize(parts.next(), &input)?;
            let x = parse_next_isize(parts.next(), &input)?;
            let y = parse_next_isize(parts.next(), &input)?;
            let chrs = get_next(parts.next(), &input)?;
            check_empty(parts.next(), &input)?;
            return Ok(Draw(index, Point::new(x, y), chrs.chars().nth(0).unwrap()));
        },
        _ => Err(ParseError::InvalidCommandError(input))
    }
}
fn get_next<T>(part: Option<T>, input: &str) -> Result<T> {
    if let Some(part) = part {
        Ok(part)
    } else {
        Err(ParseError::MissingArguments(input.to_string()))
    }
}

fn parse_next_isize(part: Option<&str>, input: &str) -> Result<isize> {
    if let Some(part) = part {
        if let Ok(part) = part.parse::<isize>() {
            Ok(part)
        } else {
            Err(ParseError::NotNumber(part.to_string()))
        }
    } else {
        Err(ParseError::MissingArguments(input.to_string()))
    }
}

fn parse_next_usize(part: Option<&str>, input: &str) -> Result<usize> {
    if let Some(part) = part {
        if let Ok(part) = part.parse::<usize>() {
            Ok(part)
        } else {
            Err(ParseError::NotNumber(part.to_string()))
        }
    } else {
        Err(ParseError::MissingArguments(input.to_string()))
    }
}

fn check_empty<T>(part: Option<T>, input: &str) -> Result<()> {
    if let Some(_) = part {
        Err(ParseError::TooManyArguments(input.to_string()))
    } else {
        Ok(())
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
    Ok(buffer.trim().to_string())
}
