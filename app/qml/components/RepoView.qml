import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Dialogs 1.3
import RustCode 1.0
import "../base"
import "../style"

Pane {
    anchors.fill: parent
    Component.onCompleted: {
        repositoriesModel.init(DatabaseFileName);
    }
    FileDialog {
        id: fileDialog
        title: "Select a Git repository to add"
        folder: shortcuts.home
        selectFolder: true
        onAccepted: {
            if (!repositoriesModel.add(fileDialog.fileUrls)) {
                addFailedMessageDialog.text = repositoriesModel.addLastError();
                addFailedMessageDialog.visible = true;
            }
        }
    }
    MessageDialog {
        id: addFailedMessageDialog
        title: "Could not add the selected folder."
        icon: StandardIcon.Critical
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
                                    text: displayName
                                    verticalAlignment: Text.AlignVCenter
                                    font.pointSize: Style.fontPointSize
                                    font.family: Style.fontName
                                    font.bold: current
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
                                repositoriesModel.setCurrent(id);
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
                font.family: Style.fontName
            }
            onClicked: {
                fileDialog.open()
            }
        }
    }
}
