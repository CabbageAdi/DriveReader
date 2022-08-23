# DriveReader

Drive benchmarking and partition listing tool, written in blazing-fast, ultra-memory safe, beautiful Rust.

## Installing

Go to the [releases](https://github.com/IDoEverything/DriveReader/releases) page and download the appropriate executable for your distribution

Linux (x86):

```
> chmod +x executable_name
> ./executable_name <args>
```

Other platforms:

* Install [cargo](https://doc.rust-lang.org/cargo/)
* Clone this repository
```
> cargo build --release    
```
* Run the executable in `/target/release` with appropriate arguments

## Usage

All commands must be executed as superuser, using `sudo` on unix-like systems or in a command prompt with elevated access on windows.

### Partition listing

```
<executable> l <disk/path>
```

Example:

```
./DiskReader l /dev/sdb
```

### Disk benchmarking

```
<executable> b <directory/path>
```

Example:

```
./DiskReader b /home/
```