## Work in progress, and actively developed

Contributions will be more than welcomed :+1:

Rust port of https://github.com/raspberrypi/usbboot/blob/master/main.c

## Usage

```shell
RUST_LOG=trace cargo run
[2025-02-11T15:41:30Z DEBUG rustpiboot] Waiting for BCM2835/6/7/2711...
[2025-02-11T15:41:30Z TRACE rustpiboot] Found device 1 idVendor=0x0a5c idProduct=0x2711
[2025-02-11T15:41:30Z TRACE rustpiboot] Bus: 2, Device: 1
[2025-02-11T15:41:30Z TRACE rustpiboot] Found candidate Compute Module...
[2025-02-11T15:41:30Z TRACE rustpiboot] Device located successfully
[2025-02-11T15:41:31Z TRACE rustpiboot] Initialised device correctly
[2025-02-11T15:41:31Z TRACE rustpiboot] Found serial number Some(3)
[2025-02-11T15:41:31Z DEBUG rustpiboot] Sending bootcode.bin
[2025-02-11T15:41:31Z TRACE rustpiboot] write_bulk sent: 24 bytes
[2025-02-11T15:41:31Z TRACE rustpiboot] Writing 105984 bytes
[2025-02-11T15:41:31Z TRACE rustpiboot] write_bulk sent: 105984 bytes

```