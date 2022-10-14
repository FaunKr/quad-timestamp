# quad-timestamp
 
This is a crate which enables you to get an unix timestamp in [miniquad](https:://crates.io/crates/miniquad)/[macroquad](https:://crates.io/crates/macroquad) while running in a WASM environment. In any other envoirment it will fallback to [chrono](https://crates.io/crates/chrono) to aquire a timestamp.
 

# Usage
Add this to your `Cargo.toml` dependencies:
```text
quad-timestamp = "0.1.0"
```
# Usage in WASM
Add file [`js/quad-timestamp.js`](js/quad-timestamp.js) to your project.
 

Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
```html
<script src="sapp_jsutils.js"></script> 
```
Done! Now you can use this crate.