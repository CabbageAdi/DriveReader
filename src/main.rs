use gptman;
use std::env;
use std::fs::{File, remove_file};
use std::io::{stdout, Write};
use std::ptr::write;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Disk Reader - Made with <3 by Exun Clan");
    if (args.len() < 2) {
        println!("Not enough arguments specified: see https://github.com/IDoEverything/DriveReader");
        return;
    }

    let command = &args[1];

    //partition list command
    if (command == "l") {
        if (args.len() < 3) {
            println!("Missing required argument: disk path");
            return;
        }

        let drive_path = &args[2];

        //load drive as file
        let mut f = match File::open(drive_path) {
            Ok(thing) => thing,
            Err(_) => {
                println!("could not open disk at path {drive_path} (could be missing root privileges)");
                return;
            }
        };

        //get data from library
        let gpt = match gptman::GPT::find_from(&mut f) {
            Ok(thing) => thing,
            Err(_) => {
                println!("could not find GPT on drive {}", drive_path);
                return;
            }
        };

        println!("Disk {}", drive_path);

        for (i, p) in gpt.iter() {
            //write size if healthy partition
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
            //detect unhealthy partitions
            else {
                let size = p.size().unwrap();
                if (size != 1) {
                    println!("Partition #{}: Unhealthy partition", i)
                }
            }
        }
    }

    //benchmarking command
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

        //check for custom benchmark values
        if (args.len() > 3) {
            if (args.len() % 2 == 0) {
                println!("Invalid arguments provided: expected even number of additional arguments.");
                return;
            }

            let num_args = args.len() - 3;
            let i = 0;

            //run custom benchmarks
            while i < num_args / 2 {
                let data_mb = &args[3 + i * 2].parse::<u128>().unwrap();
                let packet_count = &args[3 + i * 2 + 1].parse::<u128>().unwrap();
                write_repeated(&file_path, *data_mb, *packet_count);
            }
        }
        else {
            //default benchmark values
            write_repeated(&file_path, 1024, 8);
            write_repeated(&file_path, 8, 256);
        }
    }
    else {
        println!("No command found matching {}", command);
    }
}

//function to handle benchmarking with specific values
fn write_repeated(file_path: &String, data_mb: u128, packets: u128) {
    //create file for benchmarking
    let mut f = match File::create(file_path) {
        Ok(thing) => thing,
        Err(_) => {
            println!("Failed to create file in directory. Make sure the application has access.");
            return;
        }
    };

    println!("Writing {}mb {} times", data_mb, packets);

    //start overall timer
    let start = Instant::now();

    let mut min = 0;
    let mut max = 0;

    let mut i = 1;
    while i <= packets {
        //run individual timer
        let mut t = Instant::now();

        //write correct amount of data
        f.write(&vec![u8::MAX; data_mb as usize * 1024 * 1024]).expect("Error writing to file");
        //save
        f.sync_data().expect("Error saving to file");

        //collect individual time and check against highest and lowest values
        let elapsed = t.elapsed().as_millis();
        if (min == 0 || elapsed < min) {
            min = elapsed;
        }
        if (elapsed > max) {
            max = elapsed;
        }

        //progress bar
        print!("__ ");
        if (i % 20 == 0) {
            println!();
        }
        //flush output stream to make sure progress bar prints
        stdout().flush().expect("Failed to flush output");
        i += 1;
    }
    //delete file after running benchmarks
    remove_file(file_path).expect("Failed to delete file");
    let total_time = start.elapsed().as_millis();

    //format and print benchmark statistics
    println!();

    println!("Total time: {} ms | {} s", total_time, total_time / 100);
    println!("Min time: {} ms | {} s", min, min / 100);
    println!("Max time: {} ms | {} s", max, max / 100);
    println!("Average time per packet: {} ms | {} s", total_time / packets, (total_time / 100) / packets);
    println!("Average speed: {} MB/s", (packets * data_mb) / (total_time / 100));

    println!();
}
