QT += quick quickcontrols2
TEMPLATE=app
TARGET=test
CONFIG += c++14 qmltestcase release qml_debug
DEFINES += QT_DEPRECATED_WARNINGS
SOURCES += \
    tst_TestSetup.cpp
RESOURCES += qml.qrc

# This points into the shadowbuild folder
QML_IMPORT_PATH="$$OUT_PWD/../lib/release"
QML_DESIGNER_IMPORT_PATH="$$OUT_PWD/../lib/release"
