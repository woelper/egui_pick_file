## Egui file browser

Example app showing how to pick a file on both web and desktop.

The web application can be accessed here:

https://woelper.github.io/egui_pick_file/

For native:

On linux install dependencies of [rfd](https://docs.rs/rfd/latest/rfd/index.html) as applicable.
Copied from https://docs.rs/rfd/latest/rfd/#linux--bsd-backends

> GTK backend is used with the `gtk3` Cargo feature which is enabled by default. The GTK3
> backend requires the C library and development headers to be installed to build RFD. The package
> names on various distributions are:
> | Distribution | Installation Command |
> | --------------- | ------------ |
> | Fedora | dnf install gtk3-devel |
> | Arch | pacman -S gtk3 |
> | Debian & Ubuntu | apt install libgtk-3-dev |

`cargo run`


For web:

`rustup target add wasm32-unknown-unknown`

`cargo install --locked trunk`

`trunk serve --open`

There is also an alternate example implementation that uses [poll-promise](https://docs.rs/poll-promise/latest/poll_promise/) instead available [here](https://github.com/c-git/egui_file_picker_poll_promise)https://github.com/c-git/egui_file_picker_poll_promise.
