# Arrow Display

Arrow Display is a Rust Application used to display the current speed digitally. 

## Prerequisites

- Rust 1.36 or higher
- Cargo (Comes with Rust installation)

## Installation

1. Clone the repo: `git clone git@github.com:team-arrow-racing/arrow_display.git`
2. Move into the newly created directory: `cd arrow_display`

## Running

After you cloned the repository you can run the application by typing the following command into your terminal:

```
cargo run
```

The application will run on an infinite loop, displaying a random current speed every second. To stop the execution, use `Ctrl + C` keys.

## Testing

Use cargo to run the predefined tests:

```
cargo test
```

The current test examines the `show_speed` functionality in `speed_display.rs`.

## Documentation

The application consists of the following files:

- `main.rs`: Entry point of the application. It creates the display, a `SpeedDisplay` object and handles user events and window updates. It simulates the current speed with a random number generator.
- `speed_display.rs`: Contains the `SpeedDisplay` struct and its associated methods. `SpeedDisplay::show_speed` handles the graphical display of the current speed.
- `speed_display_test.rs`: Contains the tests for the `SpeedDisplay` struct.

## External Dependencies

- `embedded-graphics` for rendering the graphical user interface
- `embedded-graphics-simulator` for simulating the display
- `rand` for generating random current speeds

## Contribution

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## About Team Arrow

Team Arrow Racing Association is a volunteer organisation that designs, develops and races world-class solar powered vehicles. This repository is part of our endevour to build in the open and make our learnings accessible to anyone.

You can find our more about Team Arrow on [our website](https://www.teamarrow.com.au/).