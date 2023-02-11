# rust-opengl-sdl2-template
Starting template for OpenGL + SDL2 in Rust

Uses [gl33 0.2.1](https://docs.rs/gl33/0.2.1/gl33/), [beryllium 0.9.0](https://docs.rs/beryllium/0.9.0/beryllium/) and [fermium 22401.1.0](https://docs.rs/fermium/22401.1.0/fermium/).

# Requirements
Since SDL2 is dynamically linked, be sure to place SDL2 library alongside the main executable, e.g. on Windows put `SDL2.dll` to `target/release` (or `target/debug`).

You can get SDL2 here: https://github.com/libsdl-org/SDL/releases
