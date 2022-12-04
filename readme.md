# Bloody.rs - a small utility to control bloody mouse backlight intensity

* Currently only simple CLI mode implemented
* Tested with [AL90 Mouse](http://www.bloody.com/en/product.php?pid=10&id=100)

# Compilation

```bash
git clone git@github.com:encrawler/bloody.rs.git
cd bloody.rs
cargo run
```

# USB device access fix (linux)

* By default, you need to be root to access USB devices
* Create a new udev rule to drop this requirement for bloody mouses:

```bash
echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="09da", MODE="0666"' | sudo tee /etc/udev/rules.d/a4.rules
# Replug the mouse to reapply new udev settings
```
