# ESP32 System Monitor Display
ESP32 part of esp32-sysmon-* project;\
[Desktop part of this project](https://github.com/c4ff0e/esp32-sysmon-server) - requiered to collect metrics for this display

# ESP32 SoC requierements and support
**It is possible, that with changes to source code ESP32's without exact requiered fetures will also work**
## Requierements
- Native USB
- Xtensa CPU
- ~4MB flash

## Supported SoC's
ESP32-S3 - without changes to code/build - you can flash right away!

ESP32-S2 - with small changes to [Cargo.toml](Cargo.toml) and [.cargo/config.toml](.cargo/config.toml). These changes will be discussed [later](#hardware) in this README. Support is theoretical, not tested

# Requiered peripherals
**It is possible, that with changes to source code other peripherals will also work**

`KY-012 active buzzer` ***(optional)***\
`128x160 ST7735 SPI screen`

**Remark about  screen birghtness**\
I have built this firmware without PWM screen brightness, so brightness is always 100%.

# Suggested wiring
## ST7735 Display
| Display pin | ESP32-S3 pin | Notes |
|-------------|--------------|-------|
| VCC         | 3.3V         | Power |
| GND         | GND          | Ground|
| SCL / SCK   | GPIO12       | SPI clock|
| SDA / MOSI  | GPIO11       | SPI data|
| RES / RST / RESET| GPIO2   | Reset|
| DC / A0     | GPIO7        | Data/command|
| CS          | GPIO1        | Chip select|
| BLK / LED   | 3.3V         | Backlight, if present|

## KY-012 Buzzer
| Buzzer pin | ESP32-S3 pin | Notes |
|------------|--------------|-------|
| S          | GPIO4        |Signal |
| + / VCC    | 3.3V         |Power  |
| - / GND    | GND          |Ground |

# Flashing
## From release
**Note**: Release firmware is made only for ESP32-S3 with [suggested wiring](#suggested-wiring)\
If you want to [change the firmware to suit your wiring or preferences](#adapting-firmware-to-your-hardware-and-preferences), you should [flash from source](#from-source)

Prerequisites:\
[espflash](https://github.com/esp-rs/espflash)

Download [latest release](https://github.com/c4ff0e/esp32-sysmon-display/releases/latest)

Flash the firmware
```bash
espflash write-bin --chip esp32s3 0x0 display.bin
```
## From source
Prerequisites:

[Rust](https://rustup.rs/)\
[espup](https://github.com/esp-rs/espup)\
[espflash](https://github.com/esp-rs/espflash)

This repository targets `ESP32-S3` and uses the `esp` toolchain declared in `rust-toolchain.toml`.

### Toolchain setup
Install `espup` and the Espressif Rust toolchain:

```bash
cargo install espup --locked
espup install
```

Install `espflash`:

```bash
cargo install espflash
```

### Platform-specific notes
Linux and macOS: load the environment exported by `espup`:

```bash
. $HOME/export-esp.sh
```

### Host dependencies
Debian/Ubuntu:

```bash
sudo apt install llvm-dev libclang-dev clang
```

macOS:

```bash
brew install llvm
```
### Running and building
Clone this repository:
```bash
git clone https://github.com/c4ff0e/esp32-sysmon-display

cd esp32-sysmon-display
```

This project is currently configured for `ESP32-S3` in [.cargo/config.toml](.cargo/config.toml).

Build, flash, and open the serial monitor:
```bash
cargo run --release
```

Build only:
```bash
cargo build --release
```
Build and generate release image:
```bash
cargo build --release

espflash save-image --chip esp32s3 --merge target/xtensa-esp32s3-none-elf/release/display display.bin
```
The release bin file will be generated at project root


If you are using different hardware and/or wiring, or have personal preferences, update the configuration in [Cargo.toml](Cargo.toml), 
[.cargo/config.toml](.cargo/config.toml), [main.rs](src/bin/main.rs), [frame_mgr.rs](src/render/frame_mgr.rs) before building. 

[More info here](#adapting-firmware-to-your-hardware-and-preferences)

# Adapting firmware to your hardware and preferences
**You need to [flash from source](#from-source) to perform these changes**
## Peripherals
[main.rs](src/bin/main.rs)\
Display configuration:
```rust
// change frequency if you see visual artifacts
let display_config = Config::default()
    .with_frequency(Rate::from_mhz(40)) 
    .with_mode(Mode::_0);

// change GPIO pins to match your wiring
let cs = Output::new(
    peripherals.GPIO1, // here
    Level::High,
    OutputConfig::default()
);

let reset = Output::new(
    peripherals.GPIO2, // here
    Level::High,
    OutputConfig::default()
);

let dc = Output::new(
    peripherals.GPIO7, // here
    Level::Low,
    OutputConfig::default()
);
let spi_bus = Spi::new(peripherals.SPI2, display_config).unwrap()
.with_sck(peripherals.GPIO12) // here
.with_mosi(peripherals.GPIO11); // here

let rgb = true; // set to false if red and blue are swapped
let inverted = false; // set to true if colors look inverted
```
Buzzer configuration:
```rust
let mut beeper = Output::new(
    peripherals.GPIO4, // change this GPIO pin to match your wiring
    Level::Low,
    OutputConfig::default(),
);
// if you don't want to hear beeps at all, or you don't have a buzzer, change both values to false
const BEEP_ON_UNSUPPORTED: bool = true; // unsupported desktop hardware detected
const BEEP_ON_FAILURES: bool = true;  // server unavailable or wrong USB port
```
## Hardware
**You need to [flash from source](#from-source) to perform these changes**

To flash this firmware to ESP32-S2 change all contents of [Cargo.toml](Cargo.toml) to this:
```toml
[package]
edition      = "2024"
name         = "display"
rust-version = "1.88"
version      = "1.0.0"

[[bin]]
name = "display"
path = "./src/bin/main.rs"

[dependencies]
esp-hal = { version = "~1.0", features = ["esp32s2", "log-04", "unstable"] } 


esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32s2", "log-04"] }
log                    = "0.4.27"

critical-section = "1.2.0"
esp-backtrace = { version = "0.18.1", features = [
  "esp32s2",
  "panic-handler",
  "println",
] }
esp-println = { version = "0.16.1", features = ["esp32s2", "log-04"] }
usb-device = "0.3.2"
usbd-serial = "0.2.2"

postcard = {version = "1.1.3", default-features = false}
serde = { version = "1.0.188", default-features = false, features = ["derive"] }
heapless = {version = "0.9.2", features = ["serde"]}

st7735-lcd ="0.10.0"
embedded-graphics = "0.8.2"
embedded-hal-bus = "0.3.0"
profont = "0.7.0"

[profile.dev]
# Rust debug is too slow.
# For debug builds always builds with some optimization
opt-level = "s"

[profile.release]
codegen-units    = 1     # LLVM can perform better optimizations using a single thread
debug            = 2
debug-assertions = false
incremental      = false
lto              = 'fat'
opt-level        = 's'
overflow-checks  = false
```
and [.cargo/config.toml](.cargo/config.toml)
```toml
[target.xtensa-esp32s2-none-elf]
runner = "espflash flash --monitor --chip esp32s2"

[env]
ESP_LOG="info"

[build]
rustflags = [
  "-C", "link-arg=-nostartfiles",
]

target = "xtensa-esp32s2-none-elf"

[unstable]
build-std = ["core"]
```

## Metrics display style
Metric panel appearance is defined in [frame_mgr.rs](src/render/frame_mgr.rs).  

You can adjust:                                                                                          
- CPU, GPU, and RAM border colors and enable fill
- text colors
- font styles

Example:
```rust
const CPU_TEXT_STYLE: MonoTextStyle<'_, Rgb565> =
MonoTextStyle::new(&PROFONT_12_POINT, Rgb565::CSS_STEEL_BLUE);
// you can change text color and font size
```
