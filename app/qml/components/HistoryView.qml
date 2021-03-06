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
    property bool reload: false
    color: Style.window
    onReloadChanged: {
        if (reload) {
            historyListView.currentIndex = 0;
            reload = false;
        }
    }

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
            onTriggered: {
                selected = commitId;
            }
        }
        onCurrentItemChanged: {
            if (currentItem) {
                timerRef.restart();
                timerRef.commitId = currentItem.commitId;
            }
        }
        onCurrentIndexChanged: {
            if (currentItem) {
                timerRef.restart();
                timerRef.commitId = currentItem.commitId;
            }
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
            Rectangle {
                height: rowHeight + 200
                z: 2 // placing the highlight above the list prevents flickering
                clip: true
                color: Style.selection
                Rectangle {
                    x: 5
                    height: rowHeight
                    width: graphViewSplitter.width
                    color: "transparent"
                    GraphView {
                        anchors.fill: parent
                        graphHeight: rowHeight
                        graphWidth: graphViewSplitter.width
                        graphData: historyListView.currentItem && historyListView.currentItem.graphValue
                        isMergeNode: historyListView.currentItem && historyListView.currentItem.isMergeNode
                        isSelected: true
                        requiresUpdates: true
                    }
                }
                Rectangle {
                    y: 0
                    // 8 == width of splitter itself
                    x: graphViewSplitter.width + 8
                    width: parent.width - graphViewSplitter.width - 8
                    height: rowHeight
                    color: "transparent"
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
                                TextElement {
                                    x: 4 // not sure why, but text can get chopped off otherwise
                                    color: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem && historyListView.currentItem.summaryText
                                }
                            }
                            Rectangle {
                                id: hlAuthorRectRef
                                clip: false
                                Layout.preferredWidth: hlAuthorRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextElement {
                                    id: hlAuthorRef
                                    color: "white"
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem && historyListView.currentItem.authorText
                                }
                            }
                            Rectangle {
                                id: hlTimeRectRef
                                clip: false
                                Layout.preferredWidth: hlTimeRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextElement {
                                    id: hlTimeRef
                                    color: "white"
                                    anchors.left: parent.left
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: historyListView.currentItem && historyListView.currentItem.timeText
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
                    color: "transparent"
                    Rectangle {
                        x: 2
                        y: 2
                        anchors.margins: 2
                        width: parent.width - 4
                        height: parent.height - 4
                        color: Style.window
                        clip: true
                        Rectangle {
                            height: hlDetailsMessageRef.contentHeight + 20
                            width: parent.width
                            id: hlDetailsContentRef
                            y: 0
                            color: "transparent"
                            TextElement {
                                id: hlDetailsMessageRef
                                x: 10
                                y: -detailsScrollRef.position * height + 10
                                width: parent.width - x
                                selectableText: true
                                //anchors.margins: 10
                                //anchors.fill: parent
                                font.pointSize: Style.fontPointSize
                                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                                text: historyListView.currentItem && historyListView.currentItem.commitId + "\n\n" + historyListView.currentItem.messageText
                            }
                        }
                        CustomScrollBar {
                            id: detailsScrollRef
                            width: 15
                            anchors.right: parent.right
                            height: parent.height
                            policy: ScrollBar.AlwaysOn
                            size: parent.height / hlDetailsContentRef.height
                            visible: (parent.height / hlDetailsContentRef.height) < 1
                            stepSize: 1 / hlDetailsMessageRef.lineCount
                            captureMouseWheel: true
                            capturePositiveSide: false
                            containerOtherSize: parent.width
                            scrollTarget: historyListView.currentItem
                        }
                    }
                }
            }
        }

        delegate: Component {
            Item {
                property variant commitId: cid
                property string messageText: message
                property string summaryText: summary
                property string authorText: author
                property string timeText: time
                property variant graphValue: graph
                property bool isMergeNode: isMerge
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
                                id: graphComponentRef
                                graphHeight: parent.height
                                graphWidth: graphViewSplitter.width
                                graphData: graphValue
                                isMergeNode: rootItem.isMergeNode
                            }
                            Rectangle {
                                clip: true
                                Layout.preferredWidth: parent.width - graphViewSplitter.width - rowAuthorRectRef.width - rowTimeRectRef.width
                                Layout.fillHeight: true
                                color: "transparent"
                                TextElement {
                                    id: rowMessageRef
                                    x: 4 // not sure why, but text can get chopped off otherwise
                                    anchors.verticalCenter: parent.verticalCenter
                                    text: summary
                                }
                            }
                            Rectangle {
                                id: rowAuthorRectRef
                                clip: false
                                Layout.preferredWidth: rowAuthorRef.contentWidth + 5
                                Layout.fillHeight: true
                                color: "transparent"
                                TextElement {
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
                                TextElement {
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
    DesktopScrollbar {
        id: realScrollref
        anchors.top: parent.top
        anchors.right: parent.right
        scrollHeight: parent.height
        scrollSize: scrollRef.size
        scrollPosition: scrollRef.position
        onPositionChanged: {
            scrollRef.position = position
        }
        onStep: {
            var stepVal = down ? 1 : -1;
            var topIdx = Math.max(0, historyListView.indexAt(1, historyListView.contentY + 1) + stepVal);
            historyListView.positionViewAtIndex(topIdx, ListView.Beginning);
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
    Component {
        id: graphSplitterDraggerComponentRef
        Rectangle {
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
    }
    function getGraphSplitterHeight(top) {
        if (historyListView.highlightItem) {
            var off;
            if (top) {
                off = historyListView.highlightItem.y + rowHeight - historyListView.contentY;
                return (off + historyListView.highlightItem.height) < 0 ? historyListView.height : off;
            } else {
                off = historyListView.height - (historyListView.highlightItem.y + historyListView.highlightItem.height - historyListView.contentY);
                return off > historyListView.height ? 0 : off;
            }
        } else {
            return top ? historyListView.height : 0;
        }
    }
    QQC14.SplitView {
        orientation: Qt.Horizontal
        x: 0
        y: 0
        height: getGraphSplitterHeight(true)
        width: parent.width
        handleDelegate: graphSplitterDraggerComponentRef
        Rectangle {
            id: graphViewSplitter
            width: 150
            color: "transparent"
            onWidthChanged: {
                graphViewSplitterBottom.width = width
            }
        }
        Item { }
    }
    QQC14.SplitView {
        orientation: Qt.Horizontal
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: getGraphSplitterHeight(false)
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
        Item { }
    }
}
