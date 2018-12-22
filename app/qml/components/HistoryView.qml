import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"

Rectangle {
    property int rowHeight: 18
    property string selected: ""
    color: Style.dark
    ListView {
        id: historyListView
        anchors.fill: parent
        model: logModel
        spacing: 0
        ScrollBar.vertical: ScrollBar {
            minimumSize: 0.05
            width: 15
            policy: ScrollBar.AlwaysOn
            topPadding: 15
            bottomPadding: 15
            background: Rectangle{
                color: Style.window
            }
        }
        Timer {
            id: timerRef
            property string commitId: ""
            interval: 100; running: true; repeat: true
            onTriggered: selected = commitId
        }
        onCurrentIndexChanged: {
            timerRef.restart()
            timerRef.commitId = currentItem.commitId;
        }
        highlightMoveDuration: 1
        highlightMoveVelocity: 1
        keyNavigationEnabled: true
        boundsBehavior: Flickable.StopAtBounds
        Keys.onPressed: {
            var isUp = event.key === Qt.Key_PageUp;
            if (isUp || event.key === Qt.Key_PageDown) {
                var idx = historyListView.indexAt(1, isUp ? contentY + 10 : contentY + historyListView.height - 10);
                if (historyListView.currentIndex !== idx && idx !== -1) {
                    historyListView.currentIndex = idx;
                } else {
                    var shiftRows = (isUp ? -1 : 1) * (Math.floor(historyListView.height / rowHeight) - 2);
                    var resultIdx = historyListView.currentIndex + shiftRows
                    if (resultIdx < 0 || resultIdx > historyListView.count) {
                        shiftRows = isUp ? -1 * historyListView.currentIndex :
                                           historyListView.count - historyListView.currentIndex - 1;
                    }
                    historyListView.currentIndex += shiftRows;
                }
            }
        }
        delegate: Component {
            Item {
                property variant commitId: cid
                focus: true
                anchors.rightMargin: 15
                id: rootItem
                height: rowHeight
                anchors.left: parent.left
                anchors.right: parent.right
                // we always show the selection for now. use historyListView.focus to check if list is focused.
                // todo: check if flickering on key held is helped by using listview.highlight property.
                property var bgColor: ListView.isCurrentItem ? Style.selection : Style.window

                Rectangle {
                    color: bgColor
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
                                color: bgColor
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
                                color: bgColor
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

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        historyListView.currentIndex = index;
                        historyListView.forceActiveFocus();
                    }
                }
            }
        }
    }

    QQC14.SplitView {
        orientation: Qt.Horizontal
        anchors.fill: parent

        handleDelegate: Rectangle {
            color: "transparent"
            opacity: 1
            width: 10

            LinearGradient {
                anchors.top: parent.top
                start: Qt.point(0, 0)
                end: Qt.point(0, 20)
                height: 20
                width: 1
                gradient: Gradient {
                    GradientStop { position: 0.0; color: Style.dark }
                    GradientStop { position: 1.0; color: "transparent" }
                }
            }
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
