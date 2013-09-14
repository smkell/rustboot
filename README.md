rustboot
--------
A tiny 32 bit kernel written in Rust.

It paints the screen bright red and then displays some information. You can write. That's it:

![](http://i.imgur.com/XW8PUlM.png)

![](http://i.imgur.com/3cHXx2D.png)

I was inspired to download Rust and try to do this after seeing [zero.rs](https://github.com/pcwalton/zero.rs) - a stub that lets Rust programs run almost freestanding.

## Setup

You need a few things to run rustboot:

1. `qemu`
2. `nasm`
3. Rust's `master` branch or 0.7 release
4. a cross-compiler for i386
5. optionally, tools for arm-none-eabi

### Arch Linux

Simply install all dependencies:
```
# pacman -S qemu nasm rust
```

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install quemu
```

Install binutils from source.

I personally keep things I manually compile limited to my home directory, so
I use the `--prefix=/Users/steve` option. Put this wherever you want, of
course.

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2'
$ ./configure --target=i386-elf --prefix=/Users/steve
$ make && make install
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure --prefix=/Users/steve
$ make && make install
```

Same thing about the prefix applies.

Then, just make sure that `~/bin` is in your `PATH`, if you're using a prefix.

## Running it

To compile, simply execute `make` command.

To run, use:
```bash
$ make run	# emulate default platform (x86)
$ make run arch=arm	# run on arm
$ make debug arch=arm	# debug on arm
```
