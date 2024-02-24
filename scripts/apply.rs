#!/usr/bin/env -S cargo +nightly -Zscript
```cargo
[package]
edition = "2021"

[dependencies]
xshell = "0.2.5"
```

use xshell::{Shell, cmd};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    cmd!(sh, "...");
}