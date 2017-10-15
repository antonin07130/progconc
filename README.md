# progconc
Concurrent Programming project (http://www.i3s.unice.fr/~riveill/programmation-concurrente/)


# Prerequisites. 

## Headless compilation (no GUI)

Install Rust language on your machine (should be packed with cargo).
The easiest way to install Rust on Osx and Linux is [Rustup](https://www.rustup.rs/).

## Compilation with GUI

You need to have the `sdl2` developmenet libraries on your machine to be able to compile this program's GUI.

 * For OSX :
```
brew install sdl2
export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
```

 * For Linux :
```
sudo apt-get install libsdl2-dev
```
(if you use red hat variant, you must know hhow to do that on your system, isnt'it jedi ?)


# Run tests. 
```bash
cargo test --package progconc --lib tests
```
# Run main program (dev mode, using `cargo`). 

## Usage

You can get the most recent usage description by running 

```bash
cargo run -- --help
```

```man
USAGE:
    progconc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -m, --measure    turns on performance measurement for the selected scenario
    -V, --version    Prints version information

OPTIONS:
    -p, --persons <pow_pers>     The number of persons to generate, the program will create 2^p Persons
    -t, --scenario <scenario>    The scenario to use : 0 -> 1 thread per Person, 1 -> 4 threads to manage the Terrain, 2 -> mono-threaded
```

## Examples

### Mono threaded version with few users
```bash
cargo run -- -p2 -t2 -m
```
