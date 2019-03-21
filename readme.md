# Gitbumr [![Build status](https://ci.appveyor.com/api/projects/status/211dlbqs63w61har?svg=true)](https://ci.appveyor.com/project/stofte/gitbumr)

Qt based Git client using a rust backend. WIP.

![Log view](https://i.imgur.com/NLjuY5R.png "Log view")

# Development

Following are requirements to build the repository locally.

 - [rustc 1.33.0](https://rustup.rs/)
 - [Qt 5.12.1](https://www.qt.io/offline-installers)
 - [rust-qt-binding-generator](https://github.com/KDE/rust-qt-binding-generator) 

The project has a QtCreator project, which doubles as a project file and a
makefile for [qmake](http://doc.qt.io/qt-5/qmake-manual.html). Generated bindings
are checked into sourcecontrol so the rust-qt binding generator is not required
for just building.

## Rust+Qt bindings

The project uses `rust-qt-binding-generator`
to generate both a rust interface and a C++ interface for use by Qt. Since
generating bindings happens only when the interface changes, it's a manual
build step.

Checking out the project, and build a release executable
`cargo build --release`, then add the executable to your path. Run
`rust_qt_binding_generator binding.json` in this project root folder, to
regenerate the binding code.

## Windows requirements

Visual Studio 2017 and Native SDK (check it in the VS installer). Windows CLI build steps can be found in the
[appveyor.yml](appveyor.yml) build spec.

## Ubuntu requirements

Tested on Ubuntu 18.04.2 LTS from Windows, using PuTTY/[VcXsrv](https://sourceforge.net/projects/vcxsrv/)

 - Get rust installed: `curl https://sh.rustup.rs -sSf | sh` and then set the path
   `source $HOME/.cargo/env` (or restarting the shell should also work)
 - Qt installer and/or Rust requires these packages installed
   `libgl1-mesa-glx libx11-xcb1 libxkbcommon-x11-dev libfontconfig`
   `build-essential libxrender1 libssl-dev pkg-config libgl1-mesa-dev`
 - Download Qt installer:
   `wget http://mirrors.dotsrc.org/qtproject/archive/qt/5.12/5.12.1/qt-opensource-linux-x64-5.12.1.run`
 - Mark installer as runnable `chmod +x qt-opensource-linux-x64-5.12.1.run`
 - Install `./qt-opensource-linux-x64-5.12.1.run`
 - Ensure *Tools > QtCreator* and *Qt > Desktop gcc 64-bit* are selected

Be sure to run VcXsrv in the [right configuration](https://github.com/Microsoft/WSL/issues/2855#issuecomment-358861903)
and remember to set/export `LIBGL_ALWAYS_INDIRECT=1` in the shell. If using PuTTY,
also remember to check "Enable X11 forwarding" under *Connection > SSH > X11*,
and instead of setting the `DISPLAY` env variable in the shell, enter `:0` in
the "X display location" field.

## Automated Tests

Running the tests requires environment variables set. 

 - `TST_GIT_PATH` path to a git repository used for tests.
 - `QML2_IMPORT_PATH` is used by Qt when looking for QML plugins. If the 
repository is checked out at `C:\src\gitbumr` and shadowbuild has been configured inside
the repository, set the following path:
`QML2_IMPORT_PATH=C:\src\gitbumr\build-gitbumr-Desktop_Qt_5_12_1_MSVC2017_64bit-Release\lib\release`

## Other notes

 - `app/res/gitbumr.rc` should be windows-1252 encoded
 - `itemProperties` in `binding.json` should be sorted alphabetically to avoid
   confusion since `rust_qt_binding_generator` sorts these when generating the
   user role index used to access the values in QML.

QtCreator has some gotchas:

 - After editing the project files, manually run qmake by right-clicking the top
   node in Projects pane
 - When adding qml components, Use Tools -> QML/JS -> Reset Code Model, to fix
   IDE errors
 - [Qt+Win+OpenGL](https://wiki.qt.io/Qt_5_on_Windows_ANGLE_and_OpenGL) is host
   to a multitude of [weird issues and crashes](https://bugreports.qt.io/browse/QTBUG-46074?jql=text%20~%20%22QT_OPENGL%22%20and%20text%20~%20%22Windows%22)
