use gptman;
use std::env;
use std::fs::{File, remove_file};
use std::io::{stdout, Write};
use std::ptr::write;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) {
        println!("Disk Reader - Made with <3 by Exun");
        return;
    }

    let command = &args[1];
    if (command == "l") {
        if (args.len() < 3) {
            println!("Missing required argument: disk path");
            return;
        }

        let drive_path = &args[2];

        let mut f = match std::fs::File::open(drive_path) {
            Ok(thing) => thing,
            Err(_) => {
                println!("could not open disk at path {drive_path} (could be missing root privileges)");
                return;
            }
        };

        let gpt = match gptman::GPT::find_from(&mut f) {
            Ok(thing) => thing,
            Err(_) => {
                println!("could not find GPT on drive {}", drive_path);
                return;
            }
        };

        println!("Disk {}", &args[1]);

        for (i, p) in gpt.iter() {
            if p.is_used() {
                let mut size: f64 = (p.size().unwrap() * gpt.sector_size) as f64;
                let mut unit: &str = "bytes";
                if (size >= 1024.) {
                    size /= 1024.;
                    unit = "KB";
                }
                if (size >= 1024.) {
                    size /= 1024.;
                    unit = "MB";
                }
                if (size >= 1024.) {
                    size /= 1024.;
                    unit = "GB";
                }

                println!("Partition #{}: size = {} {}", i, size.trunc(), unit);
            }
            else {
                println!("Partition #{}: Unhealthy", i);
            }
        }

    }
    else if (command == "b") {
        if (args.len() < 3) {
            println!("Missing required argument: directory path");
            return;
        }
        let direc_path = &args[2];
        if (!direc_path.ends_with("/")) {
            println!("Directory path has to end with /");
            return;
        }

        let file_path = direc_path.as_str().to_owned() + "benchmarks.txt";

        println!("Running benchmarks");

        write_repeated(&file_path, 1024, 8);
        write_repeated(&file_path, 8, 256);
    }
    else {
        println!("No command found matching {}", command);
    }
}

fn write_repeated(file_path: &String, data_mb: u128, packets: u128) {
    let mut f = match File::create(file_path) {
        Ok(thing) => thing,
        Err(_) => {
            println!("Failed to create file in directory. Make sure the application has access.");
            return;
        }
    };

    println!("Writing {}mb {} times", data_mb, packets);

    let start = Instant::now();

    let mut min = 0;
    let mut max = 0;

    let mut i = 1;
    while i <= packets {
        let mut t = Instant::now();
        f.write(&vec![u8::MAX; data_mb as usize * 1024 * 1024]).expect("Error writing to file");
        f.sync_data().expect("Error saving to file");
        let elapsed = t.elapsed().as_millis();
        if (min == 0 || elapsed < min) {
            min = elapsed;
        }
        if (elapsed > max) {
            max = elapsed;
        }

        print!("__ ");
        if (i % 20 == 0) {
            println!();
        }
        stdout().flush().expect("Failed to flush output");
        i += 1;
    }
    remove_file(file_path).expect("Failed to delete file");
    let total_time = start.elapsed().as_millis();

    println!();
    println!("Total time: {} ms | {} s", total_time, total_time / 100);
    println!("Min time: {} ms | {} s", min, min / 100);
    println!("Max time: {} ms | {} s", max, max / 100);
    println!("Average time: {} ms | {} s", total_time / packets, (total_time / 100) / packets);

    println!("Average speed: {} MB/s", (packets * data_mb) / (total_time / 100));
}
