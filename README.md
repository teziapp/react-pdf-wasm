# react-pdf-wasm

if you are here you already know about [react-pdf](https://github.com/diegomura/react-pdf)

you probably don't need this lib, most of the things can be easily done by react-pdf itself!

react-pdf-wasm is [wasm-pdf](https://github.com/jussiniinikoski/wasm-pdf) wrapped in web worker with exposed api similar to react-pdf for simplicity

## motivation/reasons:
- working with large data or complex nested logic pdf blocks the main ui thread and app crashes especially with low end devices
- wanted more faster creation of pdf
- want to learn about wasm and rust

## limitations:
- can't pass nested function object, not supported by [web-worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm)

## other explored solutions:
- [web-worker](https://github.com/KMJ-007/react-pdf-webworker)

