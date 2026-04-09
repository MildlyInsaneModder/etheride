# etheride

A VEX V5 robot motion control library written in Rust using [vexide](https://vexide.dev/). It provides PID and Take-Back-Half (TBH) controllers alongside a configurable exit-condition manager for precise motor positioning.

## Table of Contents

- [etheride](#etheride)
  - [Table of Contents](#table-of-contents)
  - [Modules](#modules)
    - [PID Controller](#pid-controller)
    - [TBH Controller](#tbh-controller)
    - [Exit Condition Manager](#exit-condition-manager)
  - [Getting Started (Windows)](#getting-started-windows)
  - [Getting Started (macOS)](#getting-started-macos)
  - [Getting Started (NixOS)](#getting-started-nixos)
  - [Getting Started (Debian/Ubuntu Linux)](#getting-started-debianubuntu-linux)
  - [Getting Started (Fedora Linux)](#getting-started-fedora-linux)
  - [Learn](#learn)
  - [Development](#development)
    - [Compiling and uploading to a VEX V5 robot](#compiling-and-uploading-to-a-vex-v5-robot)
    - [Viewing program output](#viewing-program-output)

## Modules

### PID Controller

`src/pid.rs` implements a standard Proportional-Integral-Derivative (PID) controller.

**Key features:**
- Derivative is calculated on measurement (actual value) rather than error, preventing derivative kick on setpoint changes.
- Integral summation is gated by a configurable range (`summation_range`). Accumulation only occurs when the absolute error is within that range, or when `summation_range` is `0.0` (disabled, accumulates always).
- Integral is reset to zero whenever the error changes sign (crosses zero).

**Usage:**

```rust
use pid::{PID, PidTune};

// kp, ki, kd, summation_range
let tune = PidTune::new(0.5, 0.0, 0.1, 0.0);
let mut pid = PID::new(tune);

// Returns a voltage output (f32)
let output = pid.update(actual_position, goal_position);
motor.set_voltage(output.into()).unwrap();
```

### TBH Controller

`src/tbh.rs` implements a Take-Back-Half (TBH) velocity/position controller. TBH is a simple, tuning-friendly algorithm commonly used in VEX robotics.

**Key features:**
- When the error crosses zero, the gain `k` is halved (taken back half), preventing overshoot.
- `k` is reset to its initial value (`kinit`) whenever the goal changes, ensuring a responsive start on new targets.

**Usage:**

```rust
use tbh::TBH;

let mut tbh = TBH::new(0.5); // kinit

let output = tbh.update(actual_velocity, goal_velocity);
motor.set_voltage(output.into()).unwrap();
```

### Exit Condition Manager

`src/manager.rs` tracks elapsed time within configurable error bands and triggers an exit when a settle or timeout condition is met.

**Parameters (`ManagerParams`):**

| Parameter            | Type  | Description                                              |
|----------------------|-------|----------------------------------------------------------|
| `small_settle_range` | `f32` | Error must stay within this range for a short settle     |
| `small_settle_time`  | `u32` | Milliseconds to remain in the small range before exit    |
| `large_settle_range` | `f32` | Error must stay within this range for a large settle     |
| `large_settle_time`  | `u32` | Milliseconds to remain in the large range before exit    |
| `timeout`            | `u32` | Maximum milliseconds before the loop exits unconditionally|

**Usage:**

```rust
use manager::{Manager, ManagerParams};

// small_settle_range, small_settle_time (ms),
// large_settle_range, large_settle_time (ms), timeout (ms)
let params = ManagerParams::new(16.0, 20, 40.0, 80, 2000);
let mut manager = Manager::new(params);

while !manager.should_exit() {
    let error = goal - actual;
    manager.update(error);
    // ... drive motor
}
```

## Getting Started (Windows)

Follow the instructions [here](https://www.rust-lang.org/tools/install) to install `rustup`.

Run the following commands in Powershell to set up your PC for development on Windows.

- Switch to the `nightly` rust toolchain and add the `rust-src` component:

  ```console
  rustup default nightly
  rustup component add rust-src
  ```

- Install cargo-v5:

  ```console
  cargo install cargo-v5
  ```

## Getting Started (macOS)

Follow the instructions [here](https://www.rust-lang.org/tools/install) to install `rustup` on your Mac.

Run the following commands in a terminal window to setup development with vexide.

- Open a terminal and configure `rustup` to build for the V5's platform target:

- Switch to the `nightly` rust toolchain and add the `rust-src` component:

  ```console
  rustup default nightly
  rustup component add rust-src
  ```

- Install cargo-v5:

  ```console
  cargo install cargo-v5
  ```

## Getting Started (NixOS)

The Nix flake includes a devshell with every tool you need for building and uploading vexide projects.

There is a `.envrc` file for Nix + Direnv users.

## Getting Started (Debian/Ubuntu Linux)

Follow the instructions [here](https://www.rust-lang.org/tools/install) to install `rustup`. You may also prefer to install it from your system package manager or by other means. Instructions on that can be found [here](https://rust-lang.github.io/rustup/installation/other.html).

Run the following terminal commands to set up development on Debian or Ubuntu.

- Switch to the `nightly` rust toolchain and add the `rust-src` component:

  ```console
  rustup default nightly
  rustup component add rust-src
  ```

- Install cargo-v5:

  ```console
  cargo install cargo-v5
  ```

## Getting Started (Fedora Linux)

Run the following terminal commands to set up your PC for development on Fedora.

- Install Rust:

  ```console
  sudo dnf install rustup
  rustup-init -y --default-toolchain nightly
  ```

- Close and reopen the terminal, and finish installing vexide:

  ```console
  rustup component add rust-src
  cargo install cargo-v5
  ```

## Learn

[Check out the documentation](https://vexide.dev/docs/) on the official vexide website for walkthrough-style guides and other helpful learning resources!

An [API reference](https://docs.rs/vexide) is also provided by docs.rs.

## Development

### Compiling and uploading to a VEX V5 robot

Use the cargo-v5 terminal utility to build and upload this vexide project.

```console
cargo v5 build
```

Use a USB cable to connect to your robot brain or to your controller before using the `upload` subcommand to build and upload the project. Make sure to specify a program slot.

```console
cargo v5 upload
```

### Viewing program output

You can view panic messages and calls to `println!()` using the terminal.
Use a USB cable to connect to your robot brain or controller, then start the terminal:

```console
cargo v5 terminal
```
