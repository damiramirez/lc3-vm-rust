# lc3-vm-rust

Basic implementation of the [LC3](https://www.jmeiners.com/lc3-vm/) virtual machine in Rust. An educational computer architecture, it has a simplified instruction set compared to x86.

It supports the following instructions:

- BR
- ADD
- LD
- ST
- JSR
- JSRR
- AND
- LDR
- STR
- NOT
- LDI
- STI
- JMP
- LEA
- TRAP

## Usage

### Clone the repository

```shell
  git clone git@github.com:damiramirez/lc3-vm-rust.git && cd lc3-vm-rust
```

### Run with the example program - 2048 Game

```shell
  make run
```

There are other example programs in the `examples` directory. You can run them with the following command:

```shell
  make run FILENAME=./examples/FILE.obj
```
