# tic-tac-toe-yew-function-component
This is an extremely minimal [tic-tac-toe](https://github.com/hkBst/tic-tac-toe) webUI using [yew](https://github.com/yewstack/yew), for comparison with other Rust front-end frameworks. 

Yew supports two ways of writing components: with functions and with structs. Function components are (supposed to be) easier than struct components, but are also more limited. Struct components are based on the [Elm architecture](https://guide.elm-lang.org/architecture/) like in the [seed](https://github.com/seed-rs/seed) framework.

This is the function component version.

## how to use
Yew works fine with [trunk](https://github.com/thedodd/trunk), so install that: `cargo install trunk`.

Use `trunk serve` when developing and `trunk build --release` when deploying.
