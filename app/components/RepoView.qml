import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0

Pane {
    anchors.fill: parent
    ColumnLayout {
        anchors.fill: parent
        ScrollView {
            Layout.alignment: Qt.AlignTop
            ListView {
                Repositories {
                    id: repositoriesModel
                }
                Component {
                    id: repositoriesDelegate
                    RowLayout {
                        width: parent.width
                        height: 20
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            Label {
                                id: label
                                text: displayName
                                anchors.fill: parent
                                verticalAlignment: Text.AlignVCenter
                                font.pixelSize: 20
                            }
                        }
                    }
                }
                anchors.fill: parent
                model: repositoriesModel
                delegate: repositoriesDelegate
            }
        }
        Button {
            Layout.alignment: Qt.AlignBottom
            text: "Add"
        }
    }
}

