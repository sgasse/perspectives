# A question of the perspective

Create a warped text interactively in the browser. Watch your screen at a very flat angle to read the text.

Powered by [Rust][rust], [wasm-bindgen][wasm-bindgen] and the creates [`image`][image] and [`imageproc`][imageproc].
All interaction with the DOM is done with [wasm-bindgen][wasm-bindgen] and specifically `web-sys`.
The generation of the warped image is triggered automatically when the input box changes. We delay
the calculation a little with a _debounce pattern_ to avoid overloading the end device.

![Example of warped text](./example.png)

[rust]: https://www.rust-lang.org/
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[image]: https://github.com/image-rs/image
[imageproc]: https://docs.rs/imageproc/0.22.0/imageproc/
