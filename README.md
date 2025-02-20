# MachineID for Rust - Like .Net DeviceId

This crate is a [fork](https://github.com/jmif/machineid-rs) of the [original](https://github.com/Taptiive/machineid-rs) crate to provide some fixes and maintenance.
This Rust package is inspired by [DeviceId](https://github.com/MatthewKing/DeviceId), a .Net package to build a unique Machine ID.

## Features

- 3 Different hashing types (*MD5*, *SHA1*, *SHA256*)
- Different components to make the ID
- Support for Windows, Linux and MacOS
- No Admin privileges are required

## How to use

First add this to your Cargo.toml file

```toml
[dependencies]
machineid-crystal = "1.2.5"
```

Then, you need to define the builder variable with the hashing type you want.

For example, **SHA256**
```rust
use machineid_crystal::{IdBuilder, Encryption};

// There are 3 different hashing types: MD5, SHA1 and SHA256.
let mut builder = IdBuilder::new(Encryption::SHA256);
```

After that, you just need to add the components you want the id to have.

The available components are:

- **System UUID**: Unique identifier of your machine

- **CPU Cores**: Number of physical cores from your computer

- **OS Name**: Operative System name, i.e., linux/windows

- **Username**: The username currently being used

- **Machine Name**: The name of the machine

- **CPU ID**: The serial number of the processor

- **Drive Serial** : The serial number of the disk storing the OS.

For example, i will add the System UUID and CPU Cores
```rust
use machineid_crystal::HWIDComponent;

builder.add_component(HWIDComponent::SystemID).add_component(HWIDComponent::CPUCores);
```

Once you are ready, you just need to build the id with your key

```rust
let hwid = builder.build("mykey").unwrap();
```

## Todo

- Optimize the code
- Fix bugs and increase platform integration/stability

*Feel free to report any bug you find! ;)*
