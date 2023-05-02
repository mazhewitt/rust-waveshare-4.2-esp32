# ePaper Display Demo Application for waveshare 4.2" ePaper Display and an ESP32 microcontroller

This repository contains a demo application that demonstrates the usage of an ePaper display with the Rust programming language. It utilizes the ESP-IDF, embedded-graphics, and epd-waveshare libraries to display text and graphics on an ePaper display.

## Overview

The demo application initializes an ePaper display, configures the necessary pins and SPI communication, and then draws text and a line on the display. After updating the display with the drawn elements, the application sets the ePaper display to sleep.

Here is a high-level overview of the steps performed by the demo application:

1. Link patches for ESP-IDF compatibility.
2. Initialize necessary GPIO pins for ePaper display communication.
3. Initialize and configure SPI2 for communication with the ePaper display.
4. Initialize the ePaper display and create a display buffer.
5. Define text and graphics styles.
6. Draw text and a line on the display buffer.
7. Update the ePaper display with the new content from the buffer.
8. Set the ePaper display to sleep.

## Usage

To build and run the demo application, follow these steps:

1. Install the Rust toolchain and any necessary dependencies.
2. Clone this repository.
3. Build the application using `cargo build`.
4. Flash the application to your device using the appropriate flashing tools.

Once the application is running on your device, you should see the text "Hello Rust! from Mazda" and a line drawn on the ePaper display.

## Dependencies

- Rust programming language
- ESP-IDF library
- embedded-graphics library
- epd-waveshare library

# Wiring the ePaper Display

In order to properly use the ePaper display with this demo application, you will need to wire the display to your microcontroller according to the following connections:

**ePaper Display** | **Microcontroller**
------------------ | -------------------
VCC                | 3.3V
GND                | GND
DIN (MOSI)         | GPIO23
CLK (SCLK)         | GPIO18
CS                 | GPIO5
DC                 | GPIO22
RST                | GPIO21
BUSY               | GPIO4

Ensure that all connections are secure and that there are no shorts between any pins. Once the wiring is complete, you can proceed to build and run the demo application as described in the Usage section of this README.

## License

This demo application is licensed under the MIT License. See the LICENSE file for more information.