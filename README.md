# DriveReader

Drive benchmarking and partition listing tool, written in blazing-fast, ultra-memory safe, beautiful Rust.

## Installing

Go to the [releases](https://github.com/IDoEverything/DriveReader/releases) page and download the appropriate executable for your distribution from the latest release.

#### Linux (x86_64):

```
> chmod +x executable_name
> ./executable_name <args>
```

Tested on Linux Mint 20.3

#### Other platforms:

* Install [cargo](https://doc.rust-lang.org/cargo/)
* Clone this repository
```
> cargo build --release    
```
* Run the executable in `/target/release` with appropriate arguments

## Usage

All commands must be executed as superuser. For example, run with `sudo` on Unix-like systems or in a command prompt with elevated access on Windows.

### Partition listing

Usage template:
```
<executable> l <disk/path>
```

Example:

```
./DiskReader l /dev/sdb
```

This lists all healthy and unhealthy partitions on the specified disk (limited to GPT formatted drives)

### Disk benchmarking

Usage template:
```
<executable> b <directory/path> [custom benchmarks]
```

Simple example:

```
./DiskReader b /home/<username>
```

Without custom benchmarks specified, two are run by default:

```
8 packets of 1025mb each
265 packets of 8mb each
```

You can specify your own benchmarks by adding arguments in the following form:

```
<executable b <directory/path> <b1 data amount (mb)> <b1 packet count> <b2 data amount (mb)> <b2 packet count> ...
```

For example:

```
./DriveReader b /home/<username> 32 4 8 10
```

This runs two benchmarks, one with 32mb written in 4 packets, and one with 8bm written in 10 packets.

## Code explanation

The basic functionality is documented in the code itself through comments.

The list command uses the [gptman](https://docs.rs/crate/gptman/latest) library to obtain information about drives.

The benchmarking command creates a file in the specified directory and writes data to it in the specified form while timing it.

This all makes use of the Rust borrow checking system, as well as the excellent error handling features. This ensures that no memory leaks will take place, and most possible runtime errors will be properly caught and handled. The code compiles directly to machine language for optimal performance.