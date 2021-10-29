# A question of the perspective

Create a warped text interactively in the browser. Watch your screen at a very flat angle to read the text.

Powered by [Rust][rust], [wasm-bindgen][wasm-bindgen] and the creates [`image`][image] and [`imageproc`][imageproc]. The project can serve itself via [`warp`]. However, this is pretty wasteful, it generates a 7MB binary just to serve some static files.
