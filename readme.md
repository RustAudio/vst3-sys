# vst3-sys

A port of the VST3 API in pure Rust. 

We do not distribute the SDK nor try and wrap it in clean abstractions, just port compatiable bindings to the API which is based on COM. The full SDK can be found at sdk.steinberg.net or cloned from github [here](https://github.com/steinbergmedia/vst3sdk).

---

## Building

### Dependencies

The build process adopts `cargo make` for packaging the plugin to a `.vst3` folder.

``` shell
> cargo install cargo-make
```

### Building an example

``` shell
> cargo make again
```

This generates the following tree.

``` shell
❯ tree target/debug/plugin.vst3/
target/debug/plugin.vst3/
└── Contents
    ├── Resources
    └── x86_64-linux
        └── plugin.so
```

Either copy the `plugin.vst3` to your `vst3` folder (e.g., `~/.vst3`), or add the `target/debug` folder to your DAW search path (both approaches works as tested under arch linux with Bitwig and Reaper). To avoid crashing your DAW during a rebuild of the plugin, make sure that the plugin is un-loaded before rebuild.

### Logging

Logging/tracing can be done using the`log` facade. In the `again` example logging is done using `flexi_logger`, with rotating logs according to the environment variable "VST3_LOG_PATH". (If the environment variable is unset, logging is omitted.)

An example under Linux bash:

``` shell
> export VST3_LOG_PATH=/tmp/again.log
> bitwig-studio
```

And in another terminal:

``` shell
> ls /tmp/again.log/
BitwigPluginHost64_2020-04-24_00-13-34.log
> tail -f /tmp/again.log/BitwigPluginHost64_2020-04-24_00-13-34.log
```

Beware that the example produces extensive amount of logging information by default, so filtering may be appropriate (see. [documentation](https://docs.rs/flexi_logger/0.15.2/flexi_logger/) for further information).

---

## Completeness and Contributions

Currently this crate is missing definitions of some of the constants found in the SDK, and help covering them would be greatly appreciated. If you find something missing from the SDK please submit a PR to add it. You can also grep for `todo` to find gaps. 

This crate intentionally omits anything not a part of the COM-compatible API. 

---

## Modifications 

This crate does not use the SDK's definition of IID/TUID values, but a binary compatible equivalent from `com-rs`. This 

---

## Credits

The COM vtables/API are generated using `com-rs` from Microsoft, it has been altered to allow it to compile/generate code on MacOS and Linux targets. (Update 4/2020) MS has stripped `winapi` dependency from `com-rs` and it's closer to being suitable for cross platform development. Currently outstanding are some of the implementation details of the runtime and the calling convention of the generated vtables. 

VST is a trademark held by Steinberg Media Technologies, GMBH.  

## License

`vst3-sys` is licensed under the terms of the GNU GPLv3 license. This port is a derivative work of the original SDK, and while we do not redistribute any of the original source code, it was not made in isolation. 
