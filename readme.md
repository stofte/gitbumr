# Gitbumr [![Build status](https://ci.appveyor.com/api/projects/status/211dlbqs63w61har?svg=true)](https://ci.appveyor.com/project/stofte/gitbumr)

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
 - Visual Studio 2017 and Native SDK

Windows CLI build steps can be found in the [appveyor.yml](appveyor.yml) build spec.

QtCreator has some gotchas:

 - After editing the project file, manually run qmake by right-clicking the top node in Projects pane
 - When adding qml components, Use Tools -> QML/JS -> Reset Code Model, to fix IDE errors
 - [Qt+Win+OpenGL](https://wiki.qt.io/Qt_5_on_Windows_ANGLE_and_OpenGL) is host to a multitude of [weird issues and crashes](https://bugreports.qt.io/browse/QTBUG-46074?jql=text%20~%20%22QT_OPENGL%22%20and%20text%20~%20%22Windows%22). There are many env settings you can fiddle with here. I must `set QT_OPENGL=angle` in the project run settings, otherwise the IDE will crash upon startup, due to OpenGL shader incompatabilities or some such nonsense.