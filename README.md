eternalOS
=========

Flight control software for the Phoenix open-source quadcopter.

# A Note for macOS Users

This project does not currently actively support macOS users.  This does not mean that macOS users cannot use eternalOS; it does mean, however, that it may take some extra digging and configuring on your personal machine to get these packages to work.

The eternalOS build process has been tested on Windows 11 and Ubuntu/WSL 2.

# Building eternalOS

Building eternalOS requires the following dependencies:
- `rustup`: the Rust toolchain manager
- `cargo`: the Rust package manager
- `gcc-avr` or `avr-gcc`: a GNU compiler for AVR microcontrollers
- `avrdude`: an AVR programmer

These build instructions will focus on getting to a compiled `*.elf` file.  The *Flashing eternalOS* section below will focus on how to get the `*.elf` file onto the flight controller.

## For Linux Users

You will need to install the following packages:
- `avr-libc`
- `gcc-avr`
- `pkg-config`

This is most easily done using the following command: `sudo apt install avr-libc gcc-avr pkg-config`

## For Windows Users

I recommend using the WinAVR project (http://winavr.sourceforge.net/).  WinAVR has a Windows AVR compiler (`avr-gcc`) that is used to link the final build together into a `*.elf` that can be flashed to your flight controller.  **Note**: `avrdude` also comes with WinAVR; however the distribution of `avrdude` in WinAVR is incompatible with the rest of the toolchain required for this project.  See *Flashing eternalOS* for more details.

# Flashing eternalOS

## `ravedude`
I highly recommend installing [`ravedude`](https://github.com/Rahix/avr-hal/tree/main/ravedude), which is a wrapper on `avrdude` (the programmer) that integrates AVR programming with Cargo.  With `ravedude` installed, all you have to do is type `cargo run` in the terminal in order to compile your program.  The build instructions that follow will focus on 

## For Linux Users
Follow the installation instructions on the `ravedude` GitHub page (https://github.com/Rahix/avr-hal/tree/main/ravedude) up to `cargo install ravedude`.  There is no need to create a new project or modify eternalOS's `.cargo/config.toml` file.

## For Windows Users
WinAVR's distribution of `avrdude` is incompatible with the toolchains used to build this project.  To resolve this issue, I installed `avrdude` 7.0 from Marius Greuel's Windows `avrdude` GitHub repository (https://github.com/mariusgreuel/avrdude).  The proper installation can be found in the `/releases` directory of that repository.

## Flashing Your Flight Controller

More information coming soon!

# Thanks

Thanks to GitHub user [Rahix](https://github.com/Rahix/) for [`avr-hal`](https://github.com/Rahix/avr-hal-template) (an AVR hardware abstraction layer) and `avr-hal-template` (a `cargo-generate` template for easily creating Rust packages with embedded functionality).

Thanks to the [WinAVR project](http://winavr.sourceforge.net/) for the `avr-gcc` compiler.

Thanks to Marius Greuel's [`avrdude`](https://github.com/mariusgreuel/avrdude) package that enables Windows machines to flash AVR processors.

Thanks to Julian Gaal's [`mpu6050`](https://github.com/juliangaal/mpu6050) package as much of the code in `/mpu6050` is based on his driver.

# License

Per the [`avr-hal-template`](https://github.com/Rahix/avr-hal-template), this project is licensed under the MIT software license.  See `LICENSE` for more information.

GitHub user `Rahix` (https://github.com/Rahix/) receives credit for `/Cargo.toml`, `/Cargo.lock`, `/rust-toolchain.toml`, `/.cargo/config.toml`, and all JSONs in the `/avr-specs` directory.

All other work is intellectual property of Hobbs Bros. (copyright 2022).  Redistribution condition and warranty are covered in more detail in `LICENSE`, which enumerates the conditions on this software as per the MIT software license.