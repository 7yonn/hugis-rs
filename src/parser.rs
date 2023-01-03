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
    match input.split_once(' ') {
        None => {
            match input.as_str() {
                "print" => Ok(Print),
                "quit" => Ok(Quit),
                "help" => Ok(Help),
                "clear" => Ok(Clear),
                "list" => Ok(List),
                _ => Err(ParseError::InvalidCommandError(input))
            }
        }
        Some((command, args)) => {
            match command {
                "fill" => {
                    let chars: Vec<char> = args.chars().collect();
                    if chars.len() > 1 {
                        return Err(ParseError::TooManyArguments(args.to_string()));
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
                    match arg_vec[0] {
                        "window" => {
                            if arg_vec.len() > 3 {
                                return Err(ParseError::TooManyArguments(input));
                            }
                            if arg_vec.len() < 3 {
                                return Err(ParseError::MissingArguments(input));
                            }
                            let arg1 = arg_vec[1].parse::<isize>();
                            let arg2 = arg_vec[2].parse::<isize>();
                            let arg1= if let Ok(arg1) = arg1 { arg1 } else {
                                return Err(ParseError::NotNumber(args.to_string()));
                            };
                            let arg2= if let Ok(arg2) = arg2 { arg2 } else {
                                return Err(ParseError::NotNumber(args.to_string()));
                            };
                            if arg1 <= 0 || arg2 <= 0 {
                                return Err(ParseError::NoNonPositiveIntegers(args.to_string()));
                            }
                            return Ok(NewWin(arg1, arg2))
                        },
                        "shape" => {
                            let shape_type = arg_vec[1];
                            match shape_type {
                                "circle" => {
                                    if arg_vec.len() > 3 {
                                        return Err(ParseError::TooManyArguments(input));
                                    }
                                    if arg_vec.len() < 3 {
                                        return Err(ParseError::MissingArguments(input));
                                    }
                                    if let Ok(radius) = arg_vec[2].parse::<isize>() {
                                        return Ok(NewShape(Shape::Circle(radius)));
                                    } else {
                                        return Err(ParseError::NotNumber(arg_vec[2].to_string()));
                                    }
                                },
                                "square" => {
                                    if arg_vec.len() > 4 {
                                        return Err(ParseError::TooManyArguments(input));
                                    }
                                    if arg_vec.len() < 4 {
                                        return Err(ParseError::MissingArguments(input));
                                    }
                                    if let Ok(length) = arg_vec[2].parse::<isize>() {
                                        if let Ok(height) = arg_vec[3].parse::<isize>() {
                                            return Ok(NewShape(Shape::Square(length,height)));
                                        } else {
                                            return Err(ParseError::NotNumber(arg_vec[3].to_string()));
                                        }
                                    } else {
                                        return Err(ParseError::NotNumber(arg_vec[2].to_string()));
                                    }
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
                    let arg_vec: Vec<_> = args.split(' ').collect();
                    if arg_vec.len() > 2 {
                        return Err(ParseError::TooManyArguments(input));
                    }
                    if arg_vec.len() < 2 {
                        return Err(ParseError::MissingArguments(input));
                    }
                    let arg1 = arg_vec[0].parse::<isize>();
                    let arg2 = arg_vec[1].parse::<isize>();
                    let arg1= if let Ok(arg1) = arg1 { arg1 } else {
                        return Err(ParseError::NotNumber(args.to_string()));
                    };
                    let arg2= if let Ok(arg2) = arg2 { arg2 } else {
                        return Err(ParseError::NotNumber(args.to_string()));
                    };
                    if arg1 <= 0 || arg2 <= 0 {
                        return Err(ParseError::NoNonPositiveIntegers(args.to_string()));
                    }
                    return Ok(Resize(arg1, arg2))
                },
                "draw" => {
                    let arg_vec: Vec<_> = args.split(' ').collect();
                    if arg_vec.len() > 4 {
                        return Err(ParseError::TooManyArguments(input));
                    }
                    if arg_vec.len() < 4 {
                        return Err(ParseError::MissingArguments(input));
                    }
                    if let Ok(index) = arg_vec[0].parse::<usize>() {
                        let x = if let Ok(x) = arg_vec[1].parse::<isize>() {
                            x
                        } else {
                            return Err(ParseError::NotNumber(arg_vec[1].to_string()))
                        };
                        let y = if let Ok(y) = arg_vec[2].parse::<isize>() {
                            y
                        } else {
                            return Err(ParseError::NotNumber(arg_vec[2].to_string()))
                        };
                        if arg_vec[3].len() > 1 {
                            return Err(ParseError::TooManyArguments(input));
                        }
                        return Ok(Draw(index, Point::new(x, y), arg_vec[3].chars().nth(0).unwrap()));
                    } else {
                        return Err(ParseError::NotNumber(arg_vec[0].to_string()));
                    }
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
    Ok(buffer.trim().to_string())
}
