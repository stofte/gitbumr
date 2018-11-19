import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Dialogs 1.3
import RustCode 1.0
import "../base"

Pane {
    anchors.fill: parent
    FileDialog {
        id: fileDialog
        title: "Select a Git repository to add"
        folder: shortcuts.home
        selectFolder: true
        onAccepted: {
            var id = appModel.addRepository(fileDialog.fileUrls);
            console.log("repo id: ", id)
            if (id) {
                var idx = appModel.repositoryIndex(id);
                appModel.repositories.add(idx, fileDialog.fileUrls);
            } else {
                addFailedMessageDialog.detailedText = appModel.addRepositoryGetLastError();
                addFailedMessageDialog.visible = true;
            }
        }
    }
    MessageDialog {
        id: addFailedMessageDialog
        title: "Error"
        icon: StandardIcon.Critical
        TextItem { text: "Could not add the selected folder." }
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
                        Rectangle {
                            id: labelBackground
                            anchors.fill: parent
                            border.width: 1
                            border.color: "#F0F0F0"
                            color: "transparent"
                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: 8
                                color: "transparent"
                                TextItem {
                                    id: label
                                    text: displayName
                                    verticalAlignment: Text.AlignVCenter
                                    font.pixelSize: 12
                                    font.family: mainFont.name
                                }
                            }
                        }
                        MouseArea {
                            id: itemMouseArea
                            anchors.fill: parent
                            hoverEnabled: true
                            onEntered: {
                                labelBackground.border.color = "#D3D3D3";
                            }
                            onExited: {
                                labelBackground.border.color = "#F0F0F0";
                            }
                            onDoubleClicked: {
                                console.log('clicked: ', displayName);
                            }
                        }
                    }
                }
            }
            clip: true
            ScrollBar.vertical: ScrollBar { }
            model: appModel.repositories
            delegate: repositoriesDelegate
        }
        Button {
            TextItem {
                anchors.centerIn: parent
                text: "Add"
                font.pixelSize: 12
                font.family: mainFont.name
            }
            onClicked: {
                fileDialog.open()
            }
        }
    }
}

