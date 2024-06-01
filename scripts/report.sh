#!/bin/bash

Deps=cargo-tarpaulin rust-audit
cargo install ${Deps}
cargo tarpaulin --out Html

