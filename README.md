# CIDR TOOL

A Simple tool written in rust for simplifying operations while working with ipv4 CIDR.

Features currently supported:
- subnet explosion
- subnet filter by ip list

**note:** this is my first small project in rust so the code is far from good, any feedback is truly appreciated!

## install

currently is only possible to use this tool building it from source.

```bash
git clone https://github.com/tommcamm/cidr-tool
cd cidr-tool
cargo build --release
```

after building the release you can find all the executables in the `/target/release` folder.

## usage

```
./cidr-tool --help
cidr-tool 0.1.0
tommcamm <tomm.camm@protonmail.ch>
CLI tool written in rust for helping tasks related to ipv4 CIDR's

USAGE:
    cidr-tool [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -d, --debug      Turns on debug mode
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    contains    checks from a given subnet list, how many contains the given ip's
    explode     Explode the subnets given in input in a csv format
    help        Print this message or the help of the given subcommand(s)
```