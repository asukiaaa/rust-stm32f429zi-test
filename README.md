# `cortex-m-quickstart`

https://qiita.com/Kosuke_Matsui/items/031b2d60f3242617115e
https://zeptoelecdesign.com/rust-embedded2/
https://github.com/cmsis-svd/cmsis-svd/blob/main/data/STMicro/STM32F429.svd

エラーが長すぎて最初のを見れないときはlessを組み合わせる
https://stackoverflow.com/questions/27082748/only-show-first-screenful-of-compile-errors-in-rust-when-building-with-cargo
```
cargo test --color always 2>&1 | less -r
```

クロスビルドツールが無くてエラー
```
error[E0463]: can't find crate for `core`
  |
  = note: the `thumbv7em-none-eabihf` target may not be installed
  = help: consider downloading the target with `rustup target add thumbv7em-none-eabihf`
```

install cross compiler
```
rustup target add thumbv7em-none-eabihf
```

check latest versions
https://crates.io/search?q=cortex-m

error because missing arm-node-eabi-gdb
```
GDB executable "arm-none-eabi-gdb" was not found.
Please configure "cortex-debug.armToolchainPath" or "cortex-debug.gdbPath" correctly.```
```
     Running `arm-none-eabi-gdb -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/stm32f429zi-test`
error: could not execute process `arm-none-eabi-gdb -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/stm32f429zi-test` (never executed)

Caused by:
  No such file or directory (os error 2)
```

Install gdb-multiarch.
```sh
sudo apt install -y gdb-multiarch
```

Error
```
Launching gdb-server: openocd -c "gdb_port 50000" -c "tcl_port 50001" -c "telnet_port 50002" -s /home/asuki/rustprojects/stm32f429zi-test -f /home/asuki/.vscode/extensions/marus25.cortex-debug-1.12.1/support/openocd-helpers.tcl -f interface/stlink.cfg -f target/stm32f4x.cfg
    Please check TERMINAL tab (gdb-server) for output from openocd
Finished reading symbols from objdump: Time: 32 ms
Finished reading symbols from nm: Time: 27 ms
Failed to launch OpenOCD GDB Server: Error: spawn openocd ENOENT
```

```sh
sudo apt install -y openocd
```

openocdのエラーが出るので、デバッグコンソール似表示されるopenocdのコマンドを直実行
```
openocd -c "gdb_port 50000" -c "tcl_port 50001" -c "telnet_port 50002" -s /home/asuki/rustprojects/stm32f429zi-test -f /home/asuki/.vscode/extensions/marus25.cortex-debug-1.12.1/support/openocd-helpers.tcl -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
Open On-Chip Debugger 0.11.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
CDLiveWatchSetup
WARNING: interface/stlink-v2-1.cfg is deprecated, please switch to interface/stlink.cfg
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : Listening on port 50001 for tcl connections
Info : Listening on port 50002 for telnet connections
Info : clock speed 2000 kHz
Error: open failed
```

stm32を繋いで実行したら先に進んだ

```
 
xPSR: 0x01000000 pc: 0x080023b0 msp: 0x20030000
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x080023b0 msp: 0x20030000
  stm32f4x.cpu tpiu
    stm32f4x.cpu tpiu config (disable | ((external | internal (<filename> | <:port> | -)) (sync <port
              width> | ((manchester | uart) <formatter enable>))
              <TRACECLKIN freq> [<trace freq>]))
tpiu
  tpiu config (disable | ((external | internal (<filename> | <:port> | -)) (sync <port
            width> | ((manchester | uart) <formatter enable>)) <TRACECLKIN
            freq> [<trace freq>]))
Error: invalid subcommand "init"
  stm32f4x.cpu tpiu
    stm32f4x.cpu tpiu config (disable | ((external | internal (<filename> | <:port> | -)) (sync <port
              width> | ((manchester | uart) <formatter enable>))
              <TRACECLKIN freq> [<trace freq>]))
tpiu
  tpiu config (disable | ((external | internal (<filename> | <:port> | -)) (sync <port
            width> | ((manchester | uart) <formatter enable>)) <TRACECLKIN
            freq> [<trace freq>]))
Error: invalid subcommand "names"
/home/asuki/.vscode/extensions/marus25.cortex-debug-1.12.1/support/openocd-helpers.tcl:15: Error: 
in procedure 'CDSWOConfigure' 
at file "/home/asuki/.vscode/extensions/marus25.cortex-debug-1.12.1/support/openocd-helpers.tcl", line 15
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x080023b0 msp: 0x20030000
```

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

0. Before we begin you need to identify some characteristics of the target
  device as these will be used to configure the project:

- The ARM core. e.g. Cortex-M3.

- Does the ARM core include an FPU? Cortex-M4**F** and Cortex-M7**F** cores do.

- How much Flash memory and RAM does the target device has? e.g. 256 KiB of
  Flash and 32 KiB of RAM.

- Where are Flash memory and RAM mapped in the address space? e.g. RAM is
  commonly located at address `0x2000_0000`.

You can find this information in the data sheet or the reference manual of your
device.

In this example we'll be using the STM32F3DISCOVERY. This board contains an
STM32F303VCT6 microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 256 KiB of Flash located at address 0x0800_0000.

- 40 KiB of RAM located at address 0x2000_0000. (There's another RAM region but
  for simplicity we'll ignore it).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Set a default compilation target. There are four options as mentioned at the
   bottom of `.cargo/config`. For the STM32F303VCT6, which has a Cortex-M4F
   core, we'll pick the `thumbv7em-none-eabihf` target.

``` console
$ tail -n9 .cargo/config.toml
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
# target = "thumbv8m.base-none-eabi"   # Cortex-M23
# target = "thumbv8m.main-none-eabi"   # Cortex-M33 (no FPU)
# target = "thumbv8m.main-none-eabihf" # Cortex-M33 (with FPU)
```

3. Enter the memory region information into the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the STM32F303VCT6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
