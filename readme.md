# vst3-sys

A port of the VST3 API in pure Rust. 

We do not distribute the SDK nor try and wrap it in clean abstractions, just port compatiable bindings to the API which is based on COM. The full SDK can be found at sdk.steinberg.net or cloned from github [here](https://github.com/steinbergmedia/vst3sdk). 

## Completeness and Contributions

Currently this crate is missing definitions of some of the constants found in the SDK, and help covering them would be greatly appreciated. If you find something missing from the SDK please submit a PR to add it. You can also grep for `todo` to find gaps. 

This crate intentionally omits anything not a part of the COM-compatible API. 

## Modifications 

This crate does not use the SDK's definition of IID/TUID values, but a binary compatible equivalent from `com-rs`. This 

## Credits

The COM vtables/API are generated using `com-rs` from Microsoft, it has been altered to allow it to compile/generate code on MacOS and Linux targets. (Update 4/2020) MS has stripped `winapi` dependency from `com-rs` and it's closer to being suitable for cross platform development. Currently outstanding are some of the implementation details of the runtime and the calling convention of the generated vtables. 

VST is a trademark held by Steinberg Media Technologies, GMBH.  

## License

`vst3-sys` is licensed under the terms of the GNU GPLv3 license. This port is a derivative work of the original SDK, and while we do not redistribute any of the original source code, it was not made in isolation. 
