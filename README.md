Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# Half Caps
A simple command line tool to change half the letters to uppercase!

## Usage

```sh
$ halfcaps this is text
tHiS iS tExT
$ echo this is text | halfcaps
tHiS iS tExT
```

## Install Using Cargo
First, install Rust (using [rustup](https://rustup.rs/)).
Then, it is as simple as:
```sh
cargo install -f half_caps
```

## Build From Source
First, install Rust (using [rustup](https://rustup.rs/)).

Then, run the following to build from source:
```sh
# Create a local copy
git clone https://github.com/jojolepro/half_caps
cd half_caps

# Build from source files
cargo build --release

# (Optional) Install for the current user
mv target/release/halfcaps ~/.local/bin/inv
```

Note: On windows, you should run those commands inside of [git bash](https://gitforwindows.org/) or [Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10).

