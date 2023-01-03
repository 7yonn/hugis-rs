mod window;
mod parser;
use window::Window;
use window::*;
use std::process;
use parser::{get_input, parse_to_command};
use parser::Commands;

fn main() {
    let mut win = Window::new(10, 10);
    loop {
        command(&mut win);
    }
}

pub fn command(window: &mut Window) {
    let command = parser::command_from_input();
    match command {
        Err(e) => {
            println!("{}", e);
        },
        Ok(comm) => {
            match comm {
                Commands::Print => {
                    println!("Printing window.");
                    window.print();
                },
                Commands::Quit => {
                    println!("See you again next time!");
                    process::exit(0x0100);
                },
                Commands::Clear => {
                    println!("clearing terminal");
                    clear_terminal();
                },
                Commands::Help => {
                    println!(r##"\
help -> shows help
print -> prints current state of window
new [WIDTH] [HEIGHT] -> creates new window with specified width and height
resize [WIDTH] [HEIGHT] -> resizes window to specified width and height retaining state of the visible parts of window
draw [shape] [shape_args(x, y)] [shape_location(x, y)]
fill [CHAR] -> fills whole window with CHAR
    example "fill #"
replace [OLD_CHAR] [NEW_CHAR] -> replaces OLD_CHAR with NEW_CHAR in window
quit -> quits"##);
                }
                Commands::Fill(chr) => {
                    window.fill(chr);
                    println!("window filled with {chr}");
                },
                Commands::Replace(old_chr, new_chr) => {
                    window.replace(old_chr, new_chr);
                    println!("'{old_chr}' replaced with '{new_chr}'");
                },
                Commands::New(width, height) => {
                    window.comm_new(width, height);
                    println!("Made new window with {width} width and {height} height");
                },
                Commands::Resize(width, height) => {
                    window.resize(width, height);
                    println!("Resized window to {width} width and {height} height");
                },
                _ => {
                }
            }
        }
    }
}

fn test() {
    use std::{thread, time};
    let sleep_dur = time::Duration::from_secs(1);
    let mut window = Window::new(32,22);
    let big_circle = Shape {
        shape_type: shape_type::Circle(10),
    };
    let small_circle = Shape {
        shape_type: shape_type::Circle(6),
    };
    let tiny_circle = Shape {
        shape_type: shape_type::Circle(3),
    };
    let sqr = Shape {
        shape_type: shape_type::Square(32,4),
    };
    window.fill('.');
    window.draw(Point::new(15,13), sqr, '=');
    window.draw(Point::new(12,10), big_circle, '#');
    window.draw(Point::new(10,8), small_circle, 'a');
    window.draw(Point::new(8, 6), tiny_circle, 'o');
    window.print();
    window.resize(10, 10);
    window.print();
}
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
