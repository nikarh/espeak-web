# espeak-web

This is a primitive web application that listens on `0.0.0.0` on a given `$PORT`, and for every incoming POST request parses the body as text and calls `espeak` CLI command with it as an argument. The command call to `espeak` happens in a mutex lock, so that conflicting texts can not be sent twice.

The purpose of this project is to serve as a glue between `espeak` on my [Roborock] vacuum cleaner and [Home Assistant] without giving [Home Assistant] too much control over the vacuum.

## Related

- https://github.com/Hypfer/Valetudo/issues/114

## License
 
Except where noted (below and/or in individual files), all code in this repository is dual-licensed at your option under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

[Roborock]: https://en.wikipedia.org/wiki/Roborock
[Home Assistant]: https://www.home-assistant.io/
