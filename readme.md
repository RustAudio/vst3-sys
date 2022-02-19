# vst3-sys

A port of the VST3 API in pure Rust.

We do not distribute the SDK nor try and wrap it in clean abstractions, just port compatible bindings to the COM API. The full SDK can be found at sdk.steinberg.net or cloned from github [here](https://github.com/steinbergmedia/vst3sdk).

## Building Examples

The examples can be built using [cargo-make](https://github.com/sagiegurari/cargo-make).

```
cargo install --force cargo-make
```

The two current examples are `again` and `passthru`.

```
cargo make again 
cargo make passthru 
```

Provided is a script to package a vst3 plugin as a MacOS bundle, which requires an `Info.plist` file and `PkgInfo` to be included in the vst3 plugin directory.

## Completeness and Contributions

Currently this crate is missing definitions of some of the constants found in the SDK, and help covering them would be greatly appreciated. If you find something missing from the SDK please submit a PR to add it. You can also grep for `todo` to find gaps.

This crate intentionally omits anything not a part of the COM-compatible API.

## Credits

The COM vtables/API are generated using `com-rs` from Microsoft, it has been altered to allow it to compile/generate code on MacOS and Linux targets. (Update 4/2020) MS has stripped `winapi` dependency from `com-rs` and it's closer to being suitable for cross platform development. Currently outstanding are some of the implementation details of the runtime and the calling convention of the generated vtables.

VST is a trademark held by Steinberg Media Technologies, GMBH.  

## License

`vst3-sys` is licensed under the terms of the GNU GPLv3 license. This port is a derivative work of the original SDK, and while we do not redistribute any of the original source code, it was not made in isolation.
