# Sanka [![Build Status](https://travis-ci.org/Luminarys/Sanka.svg?branch=master)](https://travis-ci.org/Luminarys/Sanka)

Sanka is a bittorrent tracker built with both features, extensibility, and performance in mind.
It currently offers:
* Full public tracker support
* IPv6 support according to BEP 7
* Basic statistics
* Private tracker support

## Dependencies
* Rust

## Installation
    1. git clone https://github.com/Luminarys/Sanka.git && cd Sanka
    2. Configure your features in Cargo.toml. In the features section you may remove "stats" or add "private". If you do want to use private tracker features, it's recommended you write an implementation in private.rs or use an existing implentation.
    3. cargo build --release
    4. The generated executable is located at target/release/sanka

## Configuration
* Modify example_config.toml as you please. All time are in seconds

## Running
* Sanka can be run as `sanka -h` to see help options
* `sanka -c [path to config file]` will run sanka with the path to the specified config file.
* `sanka` alone will run sanka with the default configuration, which can be found in example_config.toml

Currently planned features:
* UDP tracker support
* More extensive metrics

Internally, planned additions are:
* Unit tests
* Documentation
* CI
