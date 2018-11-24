import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Dialogs 1.3
import RustCode 1.0
import "../base"
import "../style"

Pane {
    Repositories {
        id: repositoriesModel
    }
    Component.onCompleted: {
        repositoriesModel.init(DatabaseFileName);
        console.log("repo started up")
    }

    FileDialog {
        id: fileDialog
        title: "Select a Git repository to add"
        folder: shortcuts.home
        selectFolder: true
        onAccepted: {
            if (!repositoriesModel.add(fileDialog.fileUrls)) {
                addFailedMessageDialog.detailedText = repositoriesModel.addLastError();
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
                    height: 20
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
                                anchors.topMargin: 5
                                anchors.leftMargin: 2
                                anchors.rightMargin: 2
                                anchors.bottomMargin: 5
                                color: "transparent"
                                TextItem {
                                    id: label
                                    text: displayName
                                    verticalAlignment: Text.AlignVCenter
                                    font.pointSize: Style.fontPointSize
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
            interactive: false
            model: repositoriesModel
            delegate: repositoriesDelegate
        }
        Button {
            TextItem {
                anchors.centerIn: parent
                text: "Add"
                font.pointSize: Style.fontPointSize
                font.family: mainFont.name
            }
            onClicked: {
                fileDialog.open()
            }
        }
    }
}

