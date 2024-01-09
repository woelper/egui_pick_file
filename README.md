## Egui file browser

Example app showing how to pick a file on both web and desktop.

The web application can be accessed here:

https://woelper.github.io/egui_pick_file/


For native:
`cargo run`

For web:

`rustup target add wasm32-unknown-unknown`

`cargo install --locked trunk`

`trunk serve --open`