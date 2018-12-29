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
    color: Style.window
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
        highlightResizeDuration: 1
        highlightMoveDuration: 1
        highlightMoveVelocity: -1
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
        highlightFollowsCurrentItem: true
        highlight: Component{
            Item {
                height: rowHeight + 200
                z: 2 // placing the highlight above the list prevents flickering
                Rectangle {
                    y: 0
                    // 8 == width of splitter itself
                    x: graphViewSplitter.width + 8
                    width: parent.width - graphViewSplitter.width - 8
                    height: rowHeight
                    color: Style.selection

                    Rectangle {
                        anchors.fill: parent
                        // these two margin values have been set using visual inspection.
                        // the trick is to overlay the highlight row exactly above the normal text row.
                        // this requires pixel perfect layout to avoid pixel shenanigans ...
                        anchors.leftMargin: 1
                        anchors.rightMargin: 11
                        color: "transparent"
                        RowLayout {
                            anchors.fill: parent
                            Rectangle {
                                clip: true
                                Layout.preferredWidth: parent.width - hlAuthorRectRef.width - hlTimeRectRef.width
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    x: 4 // not sure why, but text can get chopped off otherwise
                                    color: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem.messageText
                                }
                            }
                            Rectangle {
                                id: hlAuthorRectRef
                                clip: false
                                Layout.preferredWidth: hlAuthorRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    id: hlAuthorRef
                                    color: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem.authorText
                                }
                            }
                            Rectangle {
                                id: hlTimeRectRef
                                clip: false
                                Layout.preferredWidth: hlTimeRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    id: hlTimeRef
                                    color: "white"
                                    anchors.left: parent.left
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem.timeText
                                }
                            }
                        }
                    }
                }

                Rectangle {
                    x: 0
                    y: rowHeight
                    height: parent.height - rowHeight
                    width: parent.width
                    color: Style.selection
                    Rectangle {
                        x: 2
                        y: 2
                        anchors.margins: 2
                        width: parent.width - 4
                        height: parent.height - 4
                        color: Style.window

                        TextEdit {
                            readOnly: true
                            selectByMouse: true
                            anchors.margins: 10
                            anchors.fill: parent
                            text: historyListView.currentItem.messageText
                        }
                    }
                }
            }
        }

        delegate: Component {
            Item {
                property variant commitId: cid
                property string messageText: message
                property string authorText: author
                property string timeText: time
                property int itemHeight: rowHeight + (ListView.isCurrentItem ? 200 : 0)
                id: rootItem
                height: itemHeight
                width: parent.width - realScrollref.width
                clip: true
                anchors.left: parent.left
                Rectangle {
                    id: mainRowRef
                    color: "transparent"
                    height: rowHeight
                    anchors.top: parent.top
                    width: parent.width
                    Rectangle {
                        anchors.fill: parent
                        anchors.leftMargin: 5
                        anchors.rightMargin: realScrollref.width
                        color: "transparent"
                        RowLayout {
                            height: parent.height
                            width: parent.width
                            GraphView {
                                graphHeight: parent.height
                                graphWidth: graphViewSplitter.width
                            }
                            Rectangle {
                                clip: true
                                Layout.preferredWidth: parent.width - graphViewSplitter.width - rowAuthorRectRef.width - rowTimeRectRef.width
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    id: rowMessageRef
                                    x: 4 // not sure why, but text can get chopped off otherwise
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: message
                                }
                            }
                            Rectangle {
                                id: rowAuthorRectRef
                                clip: false
                                Layout.preferredWidth: rowAuthorRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    id: rowAuthorRef
                                    color: Style.mid
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: author
                                }
                            }
                            Rectangle {
                                id: rowTimeRectRef
                                clip: false
                                Layout.preferredWidth: rowTimeRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextItem {
                                    id: rowTimeRef
                                    color: Style.mid
                                    anchors.left: parent.left
                                    anchors.verticalCenter: parent.verticalCenter
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

    ScrollBar {
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
        id: realScrollref
        anchors.top: parent.top
        anchors.right: parent.right
        anchors.topMargin: 15
        anchors.bottomMargin: 15
        height: parent.height - 30
        policy: ScrollBar.AlwaysOn
        enabled: true
        position: scrollRef.position
        minimumSize: 0.02
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

    // The following implements the splitters that allows resizing the graph column.
    // Without using headers, this is probably the easiest way to get column resizing.

    // to prevent the splitter from overlaying the expanded current item, the splitter
    // is two splitviews that sit (along the y axis) above and below the highlighted item.
    // since this exists outside the listview itself, we bind the heights to the required
    // expressions, such that it appears visually as if the current item is fully above
    // the splitter itself.

    // graphViewSplitter and graphViewSplitterBottom set each others widths,
    // but it seems ok with qt?

    QQC14.SplitView {
        orientation: Qt.Horizontal
        x: 0
        y: 0
        height: !historyListView.highlightItem ? 0 :
            historyListView.highlightItem.y + rowHeight - historyListView.contentY
        width: parent.width

        handleDelegate: Rectangle {
            color: "transparent"
            opacity: 1
            width: 8

            LinearGradient {
                anchors.top: parent.top
                start: Qt.point(0, 0)
                end: Qt.point(0, 20)
                height: 20
                x: parent.width / 2
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
            onWidthChanged: {
                graphViewSplitterBottom.width = width
            }
        }

        Rectangle {
            color: "transparent"
        }
    }

    QQC14.SplitView {
        orientation: Qt.Horizontal
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: !historyListView.highlightItem ? 0 :
            historyListView.height - (historyListView.highlightItem.y + historyListView.highlightItem.height - historyListView.contentY)

        handleDelegate: Rectangle {
            color: "transparent"
            opacity: 1
            width: 8
        }

        Rectangle {
            id: graphViewSplitterBottom
            width: graphViewSplitter.width
            color: "transparent"
            onWidthChanged: {
                graphViewSplitter.width = width
            }
        }

        Rectangle {
            color: "transparent"
        }
    }
}
