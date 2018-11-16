import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Dialogs 1.3
import RustCode 1.0

Pane {
    anchors.fill: parent
    Repositories {
        id: repositoriesModel
    }
    FileDialog {
        id: fileDialog
        title: "Select a Git repository to add"
        folder: shortcuts.home
        selectFolder: true
        onAccepted: {
            if (!repositoriesModel.add(fileDialog.fileUrls)) {
                addFailedMessageDialog.visible = true;
            }
        }
    }
    MessageDialog {
        id: addFailedMessageDialog
        title: "Error"
        icon: StandardIcon.Critical
        text: "Could not add the selected folder."
    }
    ColumnLayout {
        anchors.fill: parent
        ListView {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Component {
                id: repositoriesDelegate
                RowLayout {
                    width: parent.width
                    height: 30
                    Item {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Label {
                            id: label
                            text: displayName
                            verticalAlignment: Text.AlignVCenter
                            font.pixelSize: 11
                            font.family: mainFont.name
                            renderType: Text.NativeRendering
                        }
                    }
                }
            }
            clip: true
            ScrollBar.vertical: ScrollBar { }
            model: repositoriesModel
            delegate: repositoriesDelegate
        }
        Button {
            Text {
                anchors.centerIn: parent
                text: "Add"
                font.pixelSize: 11
                font.family: mainFont.name
                renderType: Text.NativeRendering
            }
            onClicked: {
                fileDialog.open()
            }
        }
    }
}

