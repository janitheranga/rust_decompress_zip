extern crate zip;
extern crate zip_extract;

use std::{fs, io};
use std::io::{Write};
use std::env::args;
use std::fs::{File};
use std::path::{Path};
use std::process::exit;
use zip::write::{SimpleFileOptions};
use zip::{CompressionMethod, ZipWriter};
use zip_extract::extract;

fn main() {

    let args: Vec<_> = args().collect();
    let flag = args[2].clone().to_lowercase();

    if args.len() < 3 {
        eprintln!("Usage: {} <fileName> <Z | U>", args[0]);
        eprintln!("To zip use 'Z' or to unzip use 'U' as 2rd argument");
        eprintln!("Use folder to zip operation");
        exit(1);
    }else {
        if flag == "z" {
            exit(zip(args[1].clone()));
        } else if flag == "u" {
            exit(unzip(args[1].clone()));
        } else {
            eprintln!("Last argument should be 'Z' or 'U'. But {flag} provided.");
        }
    }
}

fn zip(arg: String) -> i32 {
    let folder_to_zip = Path::new(&arg);
    let output_file = File::create(format!("{}.zip", folder_to_zip.display())).unwrap();
    let mut zip_archive = ZipWriter::new(output_file);

    add_dir_to_zip(&mut zip_archive, folder_to_zip, folder_to_zip).unwrap();

    zip_archive.finish().unwrap();

    0
}

fn add_dir_to_zip(zip: &mut ZipWriter<File>, dir: &Path, base_dir: &Path) -> io::Result<()>{
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_dir).unwrap();

        if path.is_dir() {
            add_dir_to_zip(zip, &path, base_dir)?;
        } else {
            zip.start_file(relative_path.to_str().unwrap(), SimpleFileOptions::default().compression_method(CompressionMethod::Deflated))?;
            io::copy(&mut File::open(&path)?, zip)?;
        }
    }

    Ok(())
}
fn unzip(arg: String) -> i32 {
    let archive_path = Path::new(&arg);
    let binding = arg.to_string();
    let binding = binding.split(".").collect::<Vec<_>>();
    let destination_path = Path::new(&binding[0]);
    let mut archive_file = File::open(archive_path).unwrap();

    extract(&mut archive_file, destination_path, true).unwrap();

    0
}