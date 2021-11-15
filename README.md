# lorawan-device

This is a work-in-progress LoRaWAN device stack. It uses the traits from
the [radio](https://github.com/rust-iot/radio-hal)
crate, and provides its implementors with a simple interface to send and receive messages:

```rust
let radio = /* ... */;

let app_eui = AppEui::new(0x0000000000000000);
let dev_eui = DevEui::new(0xC0FFEEEEEEEEEEEE);
let app_key = AppKey::new(0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA);
let credentials = Credentials::new(app_eui, dev_eui, app_key);

let mut device = Device::new_otaa(radio, credentials)
    .join()
    .expect("failed to join network")
    .into_class_a();

let mut buf = [0; MAX_PAYLOAD_SIZE];
match device.transmit("hello".as_bytes(), &mut buf).expect("failed to transmit") {
    Some((size, _)) => println!("response: {:?}", buf[0..size]),
    None => println!("no response")
}
```

## Motivation

This library doesn't work yet. For a working alternative, check
out [ivajloip/rust-lorawan](https://github.com/ivajloip/rust-lorawan). In the future, this library aims to be
opinionated and easier to use.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
