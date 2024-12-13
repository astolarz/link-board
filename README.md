To get working on Raspberry Pi:
- enabled SPI
- create file `/etc/modprobe.d/spidev.conf` with contents `options spidev bufsiz=65536`