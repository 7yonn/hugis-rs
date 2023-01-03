mod window;
mod parser;
use window::Window;
use window::*;
use std::process;
use parser::Commands;

fn main() {
    let mut win = Window::new(10, 10);
    let mut shapes: Vec<Shape> = vec![];
    loop {
        command(&mut win, &mut shapes);
    }
}

pub fn command(window: &mut Window, shapes: &mut Vec<Shape>) {
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
                Commands::List => {
                    if shapes.len() == 0 {
                        println!("There are currently no shapes");
                    } else {
                        println!("Shapes: ");
                        for (i, shape) in shapes.iter().enumerate() {
                            println!("{i}. {:?}", shape);
                        }
                    }
                },
                Commands::Quit => {
                    println!("See you again next time!");
                    process::exit(0x0100);
                },
                Commands::Clear => {
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
                Commands::NewWin(width, height) => {
                    window.comm_new(width, height);
                    println!("Made new window with {width} width and {height} height");
                },
                Commands::NewShape(shape) => {
                    println!("Added new shape to shapes\n{shape:#?}");
                    shapes.push(shape);
                },
                Commands::Resize(width, height) => {
                    window.resize(width, height);
                    println!("Resized window to {width} width and {height} height");
                },
                Commands::Draw(index, point, chr) => {
                    if index > shapes.len() {
                        println!("Given index is larger than current shapes list");
                    } else {
                        window.draw(point, shapes[index], chr);
                    }
                }
                Commands::ToDo => {
                    println!("ToDo!");
                }
                }
            }
        }
    }

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
