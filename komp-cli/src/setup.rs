use std::env;

pub fn tool_name(args_iter: &mut env::Args) -> String {
    args_iter
        .next()
        .and_then(|path| {
            path.split(std::path::MAIN_SEPARATOR)
                .last()
                .map(|v| v.to_string())
        })
        .unwrap_or("komp".to_string())
}

pub fn get_source_index(args_iter: &mut env::Args, tool_name: &str) -> usize {
    match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index >= coremidi::Sources::count() {
                    println!("Source index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            }
            Err(_) => {
                println!("Wrong source index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: {} <source-index>", tool_name);
            println!("");
            println!("Available Sources:");
            print_sources();
            std::process::exit(-1);
        }
    }
}

fn print_sources() {
    for (i, source) in coremidi::Sources.into_iter().enumerate() {
        match source.display_name() {
            Some(display_name) => println!("[{}] {}", i, display_name),
            None => (),
        }
    }
}

pub fn get_destination_index(args_iter: &mut env::Args, tool_name: &str) -> usize {
    match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index >= coremidi::Destinations::count() {
                    println!("Destination index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            }
            Err(_) => {
                println!("Wrong destination index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: {} <destination-index>", tool_name);
            println!("");
            println!("Available Destinations:");
            print_destinations();
            std::process::exit(-1);
        }
    }
}

fn print_destinations() {
    for (i, destination) in coremidi::Destinations.into_iter().enumerate() {
        match destination.display_name() {
            Some(display_name) => println!("[{}] {}", i, display_name),
            None => (),
        }
    }
}
