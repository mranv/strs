<h1 align="center">
<br>
<img src=assets/strs.jpeg>
<br>
<strong>X-platform Speedtest CLI Application on rust</strong>
</h1>

![alt text](Designer-3.jpeg)

## Features

- Cross-platform compatibility (Windows, macOS, Linux)
- Fast and efficient speed testing using the Speedtest.net API
- Simple command-line interface for ease of use
- Display results in a clear and concise format

## Prerequisites

- Rust (1.50 or later) installed on your system. You can install Rust through [rustup](https://rustup.rs/).

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/mranv/strs.git
   cd strs
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. (Optional) Install the binary globally:

   ```bash
   cargo install --path .
   ```

## Usage

After installation, you can run the speed test using the following command:

```bash
strs
```

This will initiate a speed test and display the results including download and upload speeds, as well as ping time.

## Example Output

```
Running speed test...
Download Speed: 50.25 Mbps
Upload Speed: 10.00 Mbps
Ping: 30 ms
```

## Configuration

To view more options and configurations, you can run:

```bash
strs --help
```

## Contributing

Contributions are welcome! If you have suggestions or improvements, please fork the repository and submit a pull request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Thanks to Speedtest.net for providing the API.
- Thanks to the Rust community for their support and contributions.

## Contact

For any inquiries, please reach out to [iamanubhavgain](https://github.com/mranv) at [gmail.com](mailto:iamanubhavgain@gmail.com].
