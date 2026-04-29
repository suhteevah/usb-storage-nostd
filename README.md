# usb-storage-nostd

`no_std` USB Mass Storage (BOT/SCSI) driver in Rust for bare-metal systems.

## Features

- Bulk-Only Transport (BOT) protocol: 31-byte CBW, 13-byte CSW, reset recovery
- SCSI command builders/parsers: INQUIRY, READ_CAPACITY_10, READ_10, WRITE_10, TEST_UNIT_READY, REQUEST_SENSE, MODE_SENSE
- BlockDevice trait for VFS integration
- USB Mass Storage class detection (class 0x08, subclass 0x06, protocol 0x50)
- Write-protect detection via MODE_SENSE

## License

Licensed under either of Apache License 2.0 or MIT License at your option.

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
