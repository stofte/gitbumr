# Gitbumr [![build status](https://travis-ci.org/stofte/gitbumr.svg?branch=develop)](https://travis-ci.org/stofte/gitbumr)

Qt based Git client using a rust backend.

## Rust+Qt bindings

The project uses [rust-qt-binding-generator](https://github.com/KDE/rust-qt-binding-generator) to generate
both a rust interface and a C++ interface for use by Qt. Since generating bindings happens only when the 
interface changes, it's a manual build step.

Checking out the project, and build a release executable `cargo build --release`, then add the executable
to your path. Run `rust_qt_binding_generator binding.json` in this project root folder, to regenerate the
binding code.

## Developing

The project has a QtCreator project file for convenience.

 - Qt 5.11.2
 - MSVS 2017 and Native SDK

Command line usage can be found in the [.travis.yml](.travis.yml) build spec.
