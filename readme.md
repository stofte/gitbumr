# Gitbumr [![Build status](https://ci.appveyor.com/api/projects/status/211dlbqs63w61har?svg=true)](https://ci.appveyor.com/project/stofte/gitbumr)

Qt based Git client using a rust backend. WIP.

![Log view](https://i.imgur.com/NLjuY5R.png "Log view")

# Development

Following are requirements to build the repository locally.
 - [rustc 1.33.0](https://rustup.rs/)
 - [Qt 5.12.1](https://www.qt.io/offline-installers)
 - [rust-qt-binding-generator](https://github.com/KDE/rust-qt-binding-generator)
 - Windows 10 requires Visual Studio 2017 and Native SDK
 - Ubuntu 16.04 requires `build-essential` package)

The project uses QtCreator `*.pro` files as a projectfile/build format, with the
following structure:

- `gitbumr.pro` subdirs project, specifies build dependencies
   - `app.pro` Qt/QML application, refers `lib` and builds the application binary.
   - `lib.pro` Qt library which wraps the rust library itself.
     Invokes cargo to build the rust static library as its own dependency.
     Due to the way cargo is integrated into qmake, there's some gotchas,
     so cleaning in qmake will not clean cargo (by choice, see pro file)
     and all input files should be specified in the project file so that 
     QtCreator can detect when it should invoke cargo.
   - `test.pro` Qt test app, refers `lib` components

The [appveyor.yml](appveyor.yml) file contains a dual build specification for
[continous integration](https://ci.appveyor.com/project/stofte/gitbumr), with all build steps and dependencies. CI builds
produces either a zip file (Windows) or an [AppImage](https://appimage.org/) (Ubuntu)

## Rust+Qt bindings

The project uses `rust-qt-binding-generator`
to generate both a rust interface and a C++ interface for use by Qt. Since
generating bindings happens only when the interface changes, it's a manual
build step. Generated bindings are checked into sourcecontrol so the generator
is not required when just building.

To use the generator check out the project and build a release executable
`cargo build --release`, then add the executable to your path for convinience.
Run `rust_qt_binding_generator binding.json` in the `lib` project folder to
regenerate the binding code.

## Environment variables

Running the tests requires environment variables set.

 - `TST_GIT_PATH` path to a git repository used for tests.
 - `QML2_IMPORT_PATH` is used by Qt when looking for QML plugins. If the 
repository is checked out at `C:\src\gitbumr` and shadowbuild has been configured inside
the repository, set the following path:
`QML2_IMPORT_PATH=C:\src\gitbumr\build-gitbumr-Desktop_Qt_5_12_1_MSVC2017_64bit-Release\lib\release`

*QtCreator must have QML2_IMPORT_PATH set to run the application,* as the
plugin cannot be loaded in the IDE environment otherwise. Set env variables
via *Projects > Build Environment*.

## Other notes

 - `app/res/gitbumr.rc` should be windows-1252 encoded
 - `itemProperties` in `binding.json` should be sorted alphabetically to avoid
   confusion since `rust_qt_binding_generator` sorts these when generating the
   user role index used to access the values in QML.
 - AppImages binaries might require these libs to run:
  `libgl1-mesa-glx libfontconfig libcurl3`
 - When testing Ubuntu remotely from Windows, be sure to run VcXsrv in the [right
   configuration](https://github.com/Microsoft/WSL/issues/2855#issuecomment-358861903) (display 0, uncheck native opengl).
   When using PuTTY check "Enable X11 forwarding" under
   *Connection > SSH > X11*, and instead of setting the `DISPLAY` env variable in
   the shell, enter `:0` in the "X display location" field.

QtCreator has some gotchas:

 - After editing the project files, manually run qmake by right-clicking the top
   node in Projects pane
 - When adding qml components, Use Tools -> QML/JS -> Reset Code Model, to fix
   IDE errors
 - [Qt+Win+OpenGL](https://wiki.qt.io/Qt_5_on_Windows_ANGLE_and_OpenGL) is host
   to a multitude of [weird issues and crashes](https://bugreports.qt.io/browse/QTBUG-46074?jql=text%20~%20%22QT_OPENGL%22%20and%20text%20~%20%22Windows%22)

## TODO
 - AppImages have issues loading svg/font resources
 - Build proper wrapped rust components in lib project and tests for these
