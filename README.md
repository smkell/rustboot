rustboot
--------
A small kernel written in Rust.

It paints the screen bright red and then displays some information. You can write. That's it:

![](http://i.imgur.com/XW8PUlM.png)

![](http://i.imgur.com/3cHXx2D.png)

## Setup

You need a few things to run rustboot:

1. [rust-core](https://github.com/thestinger/rust-core)
2. qemu
3. nasm
4. Rust's `master` branch or 0.9 release
5. clang
6. optionally for debugging and ARM
  * gdb
  * tmux
  * GNU cross-compiler and tools for arm-none-eabi

Firstly, update rust-core.

```bash
$ git submodule update --init
### you can also pull latest rust-core:
$ git submodule foreach git pull origin master
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure
$ make && make install
```

### Arch Linux

Simply install all dependencies:
```
# pacman -S qemu nasm rust tmux
# yaourt -S gcc-arm-none-eabi
```

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install quemu
```

Install binutils from source.

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2'
$ ./configure --target=i386-elf
$ make && make install
```

## Running it

To compile, simply execute `make` command.

To run, use:
```bash
$ make run	# emulate default platform (x86)
$ make run arch=arm	# run on ARM
$ make debug arch=arm	# debug on ARM
```
