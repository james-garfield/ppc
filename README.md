# PPC

## Table of Contents

- [ppc](#project-name)
  - [Table of Contents](#table-of-contents)
  - [Description](#description)
  - [Installation](#installation)
  - [Requirements](#requirements)

## Description
A simple gRPC server and client written in Rust.

## Installation
Run in seperate terminals 
```
$ cargo build --bin ppc-server
$ cargo build --bin ppc-client
```
OR
```
$ cargo build --bin ppc-server
$ cargo build --bin ppc
```

## Requirements
Uses mysql server. To change the mysql configuration go to `src/server.rs/` file on line 193. Assumes database and tables are already created.
