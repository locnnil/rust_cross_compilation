<h1 align="center">
     ⚙️ <a href="#" alt=""> Rust Cross Compilation</a>
</h1>

<p align="center">
  <!img alt="License" src="https://img.shields.io/badge/license-MIT-brightgreen">

  <a href="https://www.linkedin.com/in/lincoln-wallace/">
    <img alt="Done by Lincoln" src="https://img.shields.io/badge/Done%20by-Lincoln%20Wallace-blue">
  </a>

  <a href="https://doc.rust-lang.org/rustc/platform-support.html">
    <img alt="Rust language" src="https://img.shields.io/badge/Language-Rust-orange">
  </a>

</p>

## 💻 **Installation**

```bash
# Clone this repo on your computer 
$ git clone https://github.com/LOCNNIL/rust_cross_compilation.git
```

If you don't have rust on your computer install it acessing 
[Rust official site](https://www.rust-lang.org/learn/get-started)
and follow the instructions.

After rust proced runing:
```bash
# Listing all arch. suported by rust:
$ rustc --print target-list
```

_OBS: see [platform suport](https://doc.rust-lang.org/beta/rustc/platform-support.html) of Rust documentation for more details_ 


Then finding your target architecture name you can add:

```bash
# Find your architecture to cross-compile and run:
# rustup target add <your-arch>

# For my armv7l embedded computer
$ rustup target add armv7-unknown-linux-gnueabi 

# For my Raspberry Pi Zero W
$ rustup target add arm-unknown-linux-gnueabihf
```

It's necessary too point to rust where is the linker for this architecture, so
let's build a folder with the configuration options inside
the project root.
```bash
$ mkdir .cargo
$ cd .cargo
$ touch config.toml
```

_OBS: If you want made this configs definitive you can modify
directly the `~/.cargo/config` file_

<br />

## :paperclip: **Specify the Linker**

### **For my armv7l embedded computer:**

It's necessary fist too have the `gcc` (or `clang`) compiler that comes with the linker, if you don't have it's necessary to install:

```bash
# In my case for the arch armv7l:
$ sudo apt install gcc-arm-linux-gnueabi
```
Then inside the `config.toml` we put what is the `gcc` (or `clang`) linker to be used pointing by the binarie.
we point:

```toml
[target.armv7-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

(You can either specify the whole path too, in my case it would be `/usr/bin/arm-linux-gnueabi-gcc` )


### **For the Raspberry Pi Zero W:**

Clone the `raspberrypi/tools` repo into a directory named `rpi_tools`

```bash
$ git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
```

Then we point to the linker in the `config.toml` file:
```toml
[target.arm-unknown-linux-gnueabihf]
linker = "~/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

<br />

## :link: **Static linking**

Depending of which type of embedded you are handling it may can be missing
 some dependencies. One way to avoid that is using static linking.
This feat is **optional** but depending of your CoM could be 
interesting. With staticaly linkage, it's possible insert all 
dependencies of your application inside the binary. It can be 
possible adding this line:

```toml
rustflags = ["-C", "target-feature=+crt-static"]
```

So for the final config file we have:
```toml
# target config for Embedded Computer with armv7l arch
[target.armv7-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

# target config for Raspberry Pi Zero W
[target.arm-unknown-linux-gnueabihf]
linker = "/home/lincoln/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

```

<br />

## :checkered_flag: **Building and Running**

```bash
# The building using cargo
# cargo build --target <your-arch>

# For the armv7l CoM
$ cargo build --target armv7-unknown-linux-gnueabi

# For the Raspberry Pi Zero W
$ cargo build --target arm-unknown-linux-gnueabihf

# Or if you already config the arch in .cargo/config.toml
$ cargo build
```

Now you just need to upload the binary to your target device using ADB, SSH, FTP server or any other way you prefer.

Runing the for the native arch (my `x86_64` computer)

![native-compilation](https://dsm01pap001files.storage.live.com/y4mAZbWqyCTvj1BLGWXOL_HDXsN5fw_x-jEXPDbG4pZnWgdlCwCXnk_U66rStjGSNbzcUlcKAdx-rJBl7YYh9vZdS3_1gmeHbakAzvnype9h0KQQoCEw02Ds79bDJZZ9gHjRdJ3t72udTEEcVDQuCwQxooGfbZ8tYAwtmvi0iyVZH5ZPN338tztx1DixpUeyTSF?width=755&height=207&cropmode=none "native compilation")


Runing for my `armv7l` CoM:

![cross-mdm9607](https://dsm01pap001files.storage.live.com/y4m2w2YGE7BGEI5Tmyy4H3yvu4exV2nWkbqYLvz_V2Bz8l0A85l6LuiB81em7emYe_ebJ-JcRxFtcjDU08dPlbv1q6Zr5bUxBSb9w06QRSnExQ5-s9KnwoBkkj_IhP_yuxahxzf3IDaJvwRYYU5dVS7f3OYXVBsiBrH-v-HLChnoHljExOm4_r1U6_1nsGKDgn-?width=646&height=216&cropmode=none "Cross compiling for mdm9607")



Runing in Raspberry Pi Zero W (`armv6l`):

![cross-raspizerow](https://dsm01pap001files.storage.live.com/y4md1ybomk8XKeoEfvVGcTj0yGEBpAELSNSY3MI4vorJASky3-I-b9qQLmrz_yI-xL93XpQZa3CKQp9gC_3XvU-UMNgquSXjaAdmjlmQzXYS0T_5xBO5cFVa858aDAHXB-SCjHlvs2qnznzb4Fl0owCKtHYE2rgo-xz-O_v4XyAx_bFmAUGl2h9VCoZnTGMM4uJ?width=802&height=264&cropmode=none "Cross compiling for Raspberry Pi Zero W")


