import QtQuick 2.9
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.3
import RustCode 1.0
import "../base"
import "../style"

Pane {
    background: Rectangle{
        color: "transparent"
    }
    ListView {
        anchors.fill: parent
        Component {
            id: gitDelegate
            Item {
                height: 20
                anchors.left: parent.left
                anchors.right: parent.right
                Rectangle {
                    id: labelBackground
                    anchors.fill: parent
                    border.width: 1
                    border.color: "#F0F0F0"
                    color: "transparent"
                    Rectangle {
                        anchors.fill: parent
                        anchors.margins: 5
                        color: "transparent"
                        TextItem {
                            text: name
                            verticalAlignment: Text.AlignVCenter
                            font.bold: checkedout
                        }
                    }
                }
                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    onEntered: {
                        labelBackground.border.color = "#D3D3D3";
                    }
                    onExited: {
                        labelBackground.border.color = "#F0F0F0";
                    }
                    onDoubleClicked: {
                        logModel.filter(oid);
                    }
                }
            }
        }
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        model: gitModel.branches
        delegate: gitDelegate
    }
}
