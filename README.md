This is a html5 canvas2d render backend for [egui](https://github.com/emilk/egui).

This project is only for my learning, there's still some issues in it and slowly running.

The main codes are in [https://github.com/rustytsuki/egui/blob/rust-office/crates/eframe/src/web/web_painter_canvas2d.rs](https://github.com/rustytsuki/egui/blob/rust-office/crates/eframe/src/web/web_painter_canvas2d.rs)

you can run it in browser as following steps:

* Install Trunk with `cargo install --locked trunk`.
* Run `trunk serve --port 8081` to build and serve on `http://127.0.0.1:8081`. Trunk will rebuild automatically if you edit the project.
* Open `http://127.0.0.1:8081/index.html#dev` in a browser. See the warning below.
