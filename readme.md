# vst3-sys

This crate is a pure Rust port of the VST3 SDK for the development of audio plugins. The full SDK can be found at 
sdk.steinberg.net or cloned from github [here](https://github.com/steinbergmedia/vst3sdk). 

## Completeness and Contributions

Currently this crate is missing definitions of some of the constants found in the SDK, and help covering them would be 
greatly appreciated. If you find something missing from the SDK please submit a PR to add it. You can also grep for `todo`
to find out where I'm aware of gaps. 

## Structure

The crate is organized like the `pluginterfaces` folder found in the VST3 SDK. If you contribute, please reflect that
(e.g. if something is missing, find the corresponding header and add a Rust port of it). 

This crate intentionally omits everything found in `public.sdk`, it only serves to cover the API. 

## Modifications 

This crate does not use the SDK's definition of IID/TUID values, but a binary compatible equivalent from `com-rs`. 

## Credits

The COM vtables/API are generated using `com-rs` from Microsoft, it has been altered to allow it to compile/generate
code on MacOS and Linux targets. 

VST3 is a trademark held by Steinberg Media Technologies, GMBH. 

## License

`vst3-sys` is licensed under the terms of the GNU GPLv3 license. This port is a derivative work of the original SDK, and
while we do not redistribute any of the original source code it was not made in isolation. If you wish to have a compatible 
license with your own agreement with Steinberg, please contact me. 

(TL;DR VST3 licensing is complicated, GPLv3 is the lowest friction to put this into the wild). 

