#!/bin/bash

Deps=cargo-tarpaulin 
cargo install ${Deps}
cargo tarpaulin --out Html

