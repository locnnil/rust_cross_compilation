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

```bash
# Find your architecture to cross-compile and run:
# rustup target add <your-arch>
$ rustup target add armv7-unknown-linux-gnueabi 
```

It's necessary too point to rust where is the linker for this architecture, so
let's build a folder with the configuration options
```bash
$ mkdir .cargo
$ cd .cargo
$ touch config.toml
```

Then inside the `config.toml` we put what is the `gcc` (or `clang`) linker to be used pointing by the binarie:

```toml
[target.armv7-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

(You can either specify the whole path too, in my case it would be `/usr/bin/arm-linux-gnueabi-gcc` )


Another thing that is **optional** but interesting depending on your 
embedded device is the staticaly linkage, to be possible insert all dependencies
of your application inside the binary. It can be possible adding this line:

```toml
rustflags = ["-C", "target-feature=+crt-static"]
```

So for the final config file we have:
```toml
[target.armv7-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

# This lines are optional, just for save time of typing:
# --target armv7-unknown-linux-gnueabi
[build]
target = "armv7-unknown-linux-gnueabi"
```

Uploading the binary to your target device and running it you will get:

![running-cross](https://prnt.sc/S9GhZhlb2Z1z "running")

