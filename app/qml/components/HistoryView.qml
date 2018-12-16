import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import "../base"
import "../style"

Rectangle {
    color: Style.dark
    ListView {
        anchors.fill: parent
        model: logModel
        spacing: 1
        ScrollBar.vertical: ScrollBar {
            minimumSize: 0.05
        }
        boundsBehavior: Flickable.StopAtBounds
        delegate: Component {
            Item {
                id: rootItem
                height: 20
                anchors.left: parent.left
                anchors.right: parent.right
                Rectangle {
                    color: Style.window
                    anchors.fill: parent
                    Rectangle {
                        anchors.fill: parent
                        anchors.leftMargin: 5
                        color: "transparent"
                        RowLayout {
                            height: rootItem.height
                            Layout.fillWidth: true
                            GraphView {
                                graphHeight: rootItem.height
                                graphWidth: graphViewSplitter.width
                            }
                            Rectangle {
                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: message
                                }
                            }
                        }
                        // to get the author/time to clip the message column,
                        // it's positioned over the sibling rowlayouts right side.
                        // assumes later source code => higher z index
                        Rectangle {
                            anchors.right: parent.right
                            height: rootItem.height
                            width: 500 // max assumed width
                            color: "transparent"
                            Rectangle {
                                width: authorTextLabel.width + 10
                                anchors.bottom: parent.bottom
                                anchors.top: parent.top
                                anchors.right: timestampColumn.left
                                color: Style.window
                                TextItem {
                                    id: authorTextLabel
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.right: parent.right
                                    anchors.rightMargin: 5
                                    color: Style.mid
                                    text: author
                                }
                            }
                            Rectangle {
                                id: timestampColumn
                                width: timeTextLabel.width + 10
                                anchors.bottom: parent.bottom
                                anchors.top: parent.top
                                anchors.right: parent.right
                                color: Style.window
                                TextItem {
                                    id: timeTextLabel
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.right: parent.right
                                    anchors.rightMargin: 5
                                    color: Style.mid
                                    text: time
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    QQC14.SplitView {
        height: 400
        orientation: Qt.Horizontal
        anchors.fill: parent

        handleDelegate: Rectangle {
            color: Style.dark
            width: 1
        }

        Rectangle {
            id: graphViewSplitter
            width: 150
            color: "transparent"
        }

        Rectangle {
            color: "transparent"
        }
    }
}
