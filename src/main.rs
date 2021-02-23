
mod lib;
extern crate image;
use std::time::Instant; 
extern crate clap;	
use clap::{Arg, App};

/// Program usage: run with --help flag to see usage
pub fn main() {
    let matches = App::new("Datamatrix Scanner")
    .version("1.0")
    .author("Aaron Leopold <aaronleopold1221@gmail.com>")
    .about("Converts CR2 Images to JPG in Batch")
    .arg(Arg::with_name("start_dir")
        .short("s")
        .long("start_dir")
        .value_name("DIR")
        .help("Sets the starting path")
        .required(true)
        .takes_value(true))
      .arg(Arg::with_name("destination")
        .short("d")
        .long("destination")
        .value_name("TO_DIR")
        .help("Sets the path to save converted images")
        .required(true)
        .takes_value(true))
    .arg(Arg::with_name("recursive")
        .short("r")
        .long("recursive")
        .help("Sets the program to search for images at and below the start_dir")
        .required(false)
        .takes_value(false))
    .get_matches();

    let start_dir = matches.value_of("start_dir").unwrap();
    let destination = matches.value_of("destination").unwrap();
    let recursive = matches.is_present("recursive");   

    let mut converter = lib::Converter::new(start_dir, destination, recursive);

    let start = Instant::now();

    let num_files = converter.run();
    
    let end = start.elapsed();

    match num_files {
      Ok(num) => {
        println!("\nCompleted... {} files handled in {:?}.", num, end);

        if num != 0 {
            println!("Average time per image: {:?}", end / num as u32);
        }
      },

      Err(err) => println!("{:?}", err)
    } 
}

