QT += quick quickcontrols2
TEMPLATE=app
CONFIG += c++14 qmltestcase release qml_debug
DEFINES += QT_DEPRECATED_WARNINGS
SOURCES += main.cpp
RESOURCES += qml.qrc

# This points into the shadowbuild folder
QML_IMPORT_PATH="$$OUT_PWD/../lib/release"
