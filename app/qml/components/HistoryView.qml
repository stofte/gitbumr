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
            id: scrollRef
            width: 0
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
        interactive: false
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
                    onPressed: {
                        historyListView.currentIndex = index;
                        historyListView.forceActiveFocus();
                    }
                    onWheel: {
                        var isDown = wheel.angleDelta.y < 0;
                        var topIdx = Math.max(0, historyListView.indexAt(1, historyListView.contentY + 1) + 3 * (isDown ? 1 : -1));
                        historyListView.positionViewAtIndex(topIdx, ListView.Beginning);
                    }
                }
            }
        }
    }

    // the qml scrollbar element lacks up/down arrrows, and has other non-desktoppy behavior,
    // such as going to position when the scrollbar gutter is clicked outside the tracker,
    // instead of paging down/up.
    // this implementation attempts to add in the up/down arrows, but keeps the non-paging
    // behavior for now.
    // currently the implementation mimics traditional windows sematics of held scrollbar buttons:
    // 1. instantly step up/down view on mouse-down
    // 2. wait N millisecs
    // 3. if still held, step view down/up one row
    // 4. wait M millisecs
    // 5. goto step 3
    // with the assumption M < N, such that the user must wait a
    // perceptable amount before rows will start scrolling "fast"
    ScrollBar {
        id: realScrollref
        anchors.top: parent.top
        anchors.right: parent.right
        anchors.topMargin: 15
        anchors.bottomMargin: 15
        height: parent.height - 30
        policy: ScrollBar.AlwaysOn
        enabled: true
        position: scrollRef.position
        property bool manipulateList: false
        background: Rectangle {
            color: Style.window
        }
        contentItem: Rectangle {
            color: realScrollref.pressed ? Style.controlActive : Style.control
        }
        onPressedChanged: {
            manipulateList = realScrollref.pressed;
        }
        onPositionChanged: {
            if (manipulateList) {
                scrollRef.position = realScrollref.position
            }
        }
        size: scrollRef.size
        width: 15
    }

    Timer {
        id: scrollStepTimerTimeoutRef
        interval: 500; running: false; repeat: false
        onTriggered: {
            scrollStepTimerRef.running = true;
            scrollStepTimerRef.restart();
        }
    }

    Timer {
        id: scrollStepTimerRef
        interval: 60; running: false; repeat: true
        property bool isDown: false
        onTriggered: {
            var topIdx = Math.max(0, historyListView.indexAt(1, historyListView.contentY + 1) + (isDown ? 1 : -1));
            historyListView.positionViewAtIndex(topIdx, ListView.Beginning);
        }
    }

    Rectangle {
        anchors.top: parent.top
        anchors.right: parent.right
        height: 15
        width: 15
        color: Style.window
        Image {
            anchors.fill: parent
            source: scrollUpMouseRef.pressed ? "/res/svg/up-active.svg" : "/res/svg/up.svg"
        }
        MouseArea {
            id: scrollUpMouseRef
            anchors.fill: parent
            onPressed: {
                var topIdx = Math.max(0, historyListView.indexAt(1, historyListView.contentY + 1) - 1);
                historyListView.positionViewAtIndex(topIdx, ListView.Beginning);
            }
            onPressedChanged: {
                if (pressed) {
                    scrollStepTimerRef.isDown = false;
                    scrollStepTimerTimeoutRef.start();
                } else {
                    scrollStepTimerTimeoutRef.stop();
                    scrollStepTimerRef.stop();
                }
            }
        }
    }

    Rectangle {
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        height: 15
        width: 15
        color: Style.window
        Image {
            anchors.fill: parent
            rotation: 180
            source: scrollDownMouseRef.pressed ? "/res/svg/up-active.svg" : "/res/svg/up.svg"
        }
        MouseArea {
            id: scrollDownMouseRef
            anchors.fill: parent
            onPressed: {
                var topIdx = Math.max(0, historyListView.indexAt(1, historyListView.contentY + 1) + 1);
                historyListView.positionViewAtIndex(topIdx, ListView.Beginning);
            }
            onPressedChanged: {
                if (pressed) {
                    scrollStepTimerRef.isDown = true;
                    scrollStepTimerTimeoutRef.start();
                } else {
                    scrollStepTimerTimeoutRef.stop();
                    scrollStepTimerRef.stop();
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
