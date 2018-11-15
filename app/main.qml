import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

ApplicationWindow {
    visible: true
    width: 450
    height: 580

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

    Page {
        anchors.fill: parent
        header: ToolBar {
            Label {
                anchors.fill: parent
                text: qsTr("QT Test")
                font.pixelSize: 20
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
        Flickable {
            anchors.fill: parent
            ListView {
                anchors.fill: parent
                model: repositoriesModel
                delegate: repositoriesDelegate
            }
        }
    }
}
