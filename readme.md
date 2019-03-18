# Gitbumr [![Build status](https://ci.appveyor.com/api/projects/status/211dlbqs63w61har?svg=true)](https://ci.appveyor.com/project/stofte/gitbumr)

Qt based Git client using a rust backend.

## Rust+Qt bindings

The project uses [rust-qt-binding-generator](https://github.com/KDE/rust-qt-binding-generator) 
to generate both a rust interface and a C++ interface for use by Qt. Since
generating bindings happens only when the interface changes, it's a manual
build step.

Checking out the project, and build a release executable
`cargo build --release`, then add the executable to your path. Run
`rust_qt_binding_generator binding.json` in this project root folder, to
regenerate the binding code.

## Windows requirements

The project has a QtCreator project, which doubles as a project file and a
makefile for [qmake](http://doc.qt.io/qt-5/qmake-manual.html).

 - Qt 5.12.1
 - Visual Studio 2017 and Native SDK

Windows CLI build steps can be found in the [appveyor.yml](appveyor.yml) build
pec.

QtCreator has some gotchas:

 - After editing the project file, manually run qmake by right-clicking the top
   node in Projects pane
 - When adding qml components, Use Tools -> QML/JS -> Reset Code Model, to fix
   IDE errors
 - [Qt+Win+OpenGL](https://wiki.qt.io/Qt_5_on_Windows_ANGLE_and_OpenGL) is host
   to a multitude of [weird issues and crashes](https://bugreports.qt.io/browse/QTBUG-46074?jql=text%20~%20%22QT_OPENGL%22%20and%20text%20~%20%22Windows%22)

Other notes

 - `app/res/gitbumr.rc` should be windows-1252 encoded
 - `itemProperties` in `binding.json` should be sorted alphabetically to avoid
   confusion since `rust_qt_binding_generator` sorts these when generating the
   user role index used to access the values in QML.

## Ubuntu requirements

These instructions have been tested on Ubuntu 18.04.2 LTS and only cover building
the application.

 - Qt requires these packages installed `libgl1-mesa-glx libx11-xcb1 libxkbcommon-x11-dev`
 - Download Qt installer: `wget http://mirrors.dotsrc.org/qtproject/archive/qt/5.12/5.12.1/qt-opensource-linux-x64-5.12.1.run`
 - Mark installer as runnable `chmod +x qt-opensource-linux-x64-5.12.1.run`
 - Install `./qt-opensource-linux-x64-5.12.1.run`
 - `curl https://sh.rustup.rs -sSf | sh` and then set the path `source $HOME/.cargo/env`
 - Rust backend further requires `libssl-dev pkg-config`

The Qt toolkit on Linux seems to refer to GCC, but who knows. For myself, 
QtCreator kept looking for `clang++` which I installed in addition to the 
previously listed dependencies.
