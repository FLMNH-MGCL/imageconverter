use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use image::{ ColorType};
use std::fs::create_dir_all;
use std::path::PathBuf;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::Once;

extern crate imagepipe;
extern crate rawloader;
extern crate error_chain;

pub mod errors {
    use error_chain::{
        error_chain,
    };

    error_chain! {}
}

extern crate glob;
use glob::{glob_with, MatchOptions};
use rayon::prelude::*;

static START: Once = Once::new();

/// Ensure the passed in path is valid (exists and is a directory), will exit with code 1 on invalid.
/// If create = true, will create directory instead of erroring out
fn path_exists(path: &str, create: bool) {
    println!("Checking the existence of {}...", path);
    let path_obj = Path::new(path);

    if (!path_obj.exists() || !path_obj.is_dir()) && !create {
        println!("failed! The path either doesn't exist in the filesystem or is not a directory: {}", path);
        println!("The program will now exit...");
        std::process::exit(1);
    } else if (!path_obj.exists() || !path_obj.is_dir()) && create {
        create_dir_all(path).expect("Could not create directory!");
        println!("created destination path...\n");
    } else {
        println!("passed!\n");
    }
}

pub struct Converter {
    start_dir: String,
    destination: String,
    recursive: bool
}

impl Converter {
    pub fn new(start_dir: &str, destination: &str, recursive: bool) -> Self {
        Self {
            start_dir: String::from(start_dir),
            destination: String::from(destination),
            recursive
        }
    }

    /// Returns nothing - checks installation of required libraries and the validity of
    /// the passed in path argument
    ///
    /// # Arguments
    ///
    /// * `path` - A str filesystem path, the starting directory to scan images
    /// * `destination` - A str filesystem path, the directory to store converted images
    fn sanity_checks(&self) {
        // check_installations();
        path_exists(self.start_dir.as_str(), false);
        path_exists(self.destination.as_str(), true);
    }

    /// Returns Result - Ok on successful read/write of source/converted images
    /// 
    /// # Arguments
    /// 
    /// * `source` - A PathBuf representing the target .CR2 image to convert
    fn imagepipe(&self) -> Result<(), &'static str> {

        let file = "./src/sample1.cr2";
        let filejpg = "./src/output_pipeline.jpg";

        println!("Loading file \"{}\" and saving it as \"{}\"", file, filejpg);

        let image = rawloader::decode_file(file);

        if image.is_err() {
            return Err("Could not decode file...");
        }

        let decoded_wrapped = imagepipe::simple_decode_8bit(file, 0, 0);

        if decoded_wrapped.is_err() {
            return Err("Could not decode file...");
        }

        let uf =  File::create(filejpg);

        if uf.is_err() {
            return Err("Could not create JPG file...");
        }

        let mut f = BufWriter::new(uf.unwrap());
        let decoded = decoded_wrapped.unwrap();

        let mut jpg_encoder = image::jpeg::JpegEncoder::new_with_quality(&mut f, 100);
        jpg_encoder
            .encode(&decoded.data, decoded.width as u32, decoded.height as u32, ColorType::Rgb8)
            .expect("Encoding image in JPEG format failed.");

        Ok(())
    }

    /// Returns Result - Ok on successful read/write of source/converted images
    /// 
    /// # Arguments
    /// 
    /// * `source` - A PathBuf representing the target .CR2 image to convert
    fn magick(&self, source: &PathBuf) -> Result<(), &'static str>  {
        START.call_once(|| {
            magick_wand_genesis();
        });
        
        let wand = MagickWand::new();

        let stem = source.file_stem().unwrap();
        let target = format!("{}/{}.jpg", self.destination.as_str(), stem.to_str().unwrap());

        wand.read_image(source.to_str().unwrap())?;
        wand.write_image(target.as_str())?;

        Ok(())
    }

    /// Returns Result - Ok on successful read/write of source/converted images
    /// 
    /// This function calls either of the two wrapper functions for converting images:
    /// magick will be attempted first, on an Err imagepipe will be attempted afterwards.
    /// 
    /// # Arguments
    /// 
    /// * `source` - A PathBuf representing the target .CR2 image to convert
    fn convert(&self, source: &PathBuf) -> Result<(), &'static str> {
        match self.magick(source) {
            Ok(_) => Ok(()),
            Err(_) => {
                println!("Could not convert image with magick, trying pipeline library...");
                return match self.imagepipe() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Could not convert image")
                }
            }
        }
    }

    fn generate_error_message(&self, path: &str, error: &str) -> String{
        format!("{}: {}", path, error)
    }

    /// Returns Result - Sucessful execution
    /// 
    /// This is the main driver for the Converter struct
    pub fn run<'a>(&mut self) -> Result<usize, &'a str> {
        self.sanity_checks();

        println!("Searching for CR2 images...");

        let glob_string = if self.recursive {
            format!("{}/**/*.cr2", self.start_dir)
        } else {
            format!("{}/*.cr2", self.start_dir)
        };
        
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let files: Vec<_> = glob_with(glob_string.as_str(), options)
            .expect("There was an error configuring the Regex used in the Glob")
            .filter_map(|x| x.ok())
            .collect();

        if files.len() == 0 {
            error_chain::bail!("No .CR2 files could be found");
        } else {
            println!("{} images found...", files.len());
        }

        println!("\nStarting conversions on multiple threads...");
        
        let image_failures: Vec<_> = files
            .par_iter()
            .map(|path| {
                self.convert(path)
                    .map_err(|e| self.generate_error_message(path.to_str().unwrap(), e))
            })
            .filter_map(|x| x.err())
            .collect();

        println!("Completed...");
        println!("\n{} failed conversions occurred...", image_failures.len());

        Ok(files.len())
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_recursive() {
        let mut converter = Converter::new("./src/test_images", "./src/destination", false);

        let ret = converter.run();

        assert_eq!(ret, Ok(1));
    }
}
