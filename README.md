# rustfire

`rustfire` is a Rust library for interacting with Wayfire, a Wayland compositor. This project provides functionalities to communicate with Wayfire using IPC, manage views, outputs, and configurations, and more.

## Features

- **List and Manage Views**: Retrieve information about and manage views.
- **List and Manage Outputs**: Get details about outputs connected to Wayfire.
- **Retrieve Configuration**: Access Wayfire's configuration details.
- **Handle Input Devices**: Get information about input devices.
- **Manage Workspaces**: Retrieve workspace details and manage workspace sets.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Installation

To include `rustfire` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
rustfire = "0.1.0"  # Replace with the actual version
```

API Methods

    connect: Establishes a connection to the Wayfire socket.
    send_json: Sends a JSON message to Wayfire and returns the response.
    read_message: Reads a JSON message from Wayfire.
    list_views: Retrieves a list of views.
    list_outputs: Retrieves a list of outputs.
    list_wsets: Retrieves a list of workspace sets.
    list_input_devices: Retrieves a list of input devices.
    get_configuration: Retrieves Wayfire's configuration.
    get_option_value: Retrieves the value of a specific configuration option.
    get_output: Retrieves information about a specific output.
    get_view: Retrieves information about a specific view.
    get_focused_view: Retrieves information about the currently focused view.
    get_focused_output: Retrieves information about the currently focused output.

Contributing

If you want to contribute to the rustfire project, follow these steps:

    Fork the repository.
    Create a new branch for your feature or bug fix.
    Make your changes and test them.
    Submit a pull request with a detailed description of your changes.

License

rustfire is licensed under the MIT License.
```
