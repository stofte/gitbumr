# Gitbumr [![build status](https://travis-ci.org/stofte/gitbumr.svg?branch=develop)](https://travis-ci.org/stofte/gitbumr)

Terminal based git client written in Rust.

## Developing on Windows WSL

Due to the rust crates used, this cannot build on Windows natively, and this limits the usefulness of VSCode on Windows. Instead, use [Ubuntu version of `code`](https://code.visualstudio.com/docs/setup/linux) and [VcXsrv](https://sourceforge.net/projects/vcxsrv/), an updated X server for Windows.

- Use `cargo run` to build and run
- TTY supporting console (only WSL Bash console tested)
- rustc 1.30.0
- libsqlite3-dev
- gcc
