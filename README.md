# Link Board
This is a project to display real time Link light rail train locations on WS2812-compatible LED displays, such as NeoPixels, written entirely in Rust. It uses the One Bus Away API to query the location data for the trains, which requires an API key (see requirements). 

This was originally written to run on a Raspberry Pi, but I have since switched to running it on ESP32-based microprocessors. It will likely still run on a Raspberry Pi, but I have not tested it on one in a while, so use at your own risk. That said, an ESP32, Raspberry Pi, or even WS2812/NeoPixel is not required to run this code, as there is a rudimentary command line output of the light rail data.

## Requirements
- A Puget Sound One Bus Away API key. Information on how to obtain a key can be found [here](https://www.soundtransit.org/help-contacts/business-information/open-transit-data-otd). The site says to allow for up to 20 business days to get a key, but I got mine in about 20 miuntes (during business hours). YMMV. API documentation can be found [here](https://developer.onebusaway.org/api/where).

## Hardware

### Chips
I have been running this on a ESP32-WROOM-32 and ESP32-S3-WROOM with no issues. I expect this should work with any ESP32 variant, though build targets may need to be adjusted.

I initially started this project on a Raspberry Pi 4, but the LEDs were lit up erratically. In hindsight, I suspect that was because I didn't know that I needed a critical section around the code where the LED data was written out, but I haven't had a chance to test the new code on my Raspberry Pi due to hardware failure (probably unrelated...).

### LEDs
I started out prototyping this on an out-of-the-box WS2812 144 count LED strip. The code is still in the project (`./link-board/src/display/strip_display.rs`), but hasn't been updated in a while; it deliberately skips over any 2 line trains.

I then thought that NeoPixel Dot Strand LEDs (at 4 inch pitch) would work well for this project, but discovered that the enclosure around the LEDs was a bit too big and unwieldy for my final display. Before I realized that, I wrote up `./link-board/src/display/string_display.rs`, which is largely based of the `strip_display.rs` version, but without any of the buffer LEDs.

I next considered using individual through-hole NeoPixels, but decided that was way more soldering that I really wanted to do, so I finally decided to cut up the LED strips I had and solder those back together to get the shape I wanted (note: it might have been easier and cleaner to have just turned the strips on their sides and use something to block the light from shining too far...). The code for this is in `./link-board/src/display/map_display.rs`.

The important thing to get any of these display types working is that they support the WS2812 format. Other than that, it doesn't matter if it's a stip, strand, individual through-hole LEDs, or whatever. The "map display" is indexed to the cut-up and re-soldered LED strips that I hacked together for my display, so you will likely want to edit the index values for your own situation. I basically cut off a bit of the strip that fit my design and coded it up to fit afterward. In the future I may convert my hard-coded values to something that reads in a CSV or other format in order to be more generic.

## Features
- `default`: headless
- `headless`: Meant to run on hardware without LEDs, displaying the data on the command line only as a row of colored rectangles.
- `rpi`: (untested in latest) Run on Raspberry Pi hardware with data connected to MOSI pin.
- `esp32`: Enables running on a ESP32 based microcontroller. Tested on ESP32 and ESP32-S3 hardware.

## Running on ESP32
- Ensure the proper target in `./link-board-esp-idf/.cargo/config.toml` is set for your chip. You may need to add the target for your particular chip.
- Create a `.env` file in the root folder with your `ONEBUSAWAY_API_KEY`, `WIFI_SSID`, and `WIFI_PASSWORD`. Optionally include the `LINK_BOARD_DISPLAY_TYPE` (default 0: strip display), `STATIONS_ONLY` (default false), or `RUST_LOG` level (default error). See `.env.example`.
- From the `./link-board-esp-idf/` directory, run `cargo run --release`
- Note: there is a bug in the esp32s3 target that requires stating the flash size with the `--flash-size` flag. I have added this to the `runner` field in the supplied config.toml. I am not sure if this issue is present on other ESP32 chips, but if you see errors when flashing the chip, this may be the reason.

## Raspberry Pi (not recently tested/use at own risk)
These instructions assume you are working with Raspberry Pi OS installed on a SD card using the Raspberry Pi Imager app.

To get working on Raspberry Pi (note: untested in a while):
- enable SPI (can be done in the Raspberry Pi Imager app when creating the image for an SD card)
- create file `/etc/modprobe.d/spidev.conf` with contents `options spidev bufsiz=65536`
- append `spidev.bufsiz=65536` to `/boot/firmware/cmdline.txt`

### Acknowledgements
The ESP32 wifi connection code comes from the [esp-rs std-training](https://github.com/esp-rs/std-training/blob/main/common/lib/wifi/src/lib.rs) repo.