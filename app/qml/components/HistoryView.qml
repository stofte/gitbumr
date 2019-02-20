import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"
import "virtuallist"
import "scrollbar2"

Rectangle {
    property int rowHeight: 18
    property string selected: ""
    property bool reload: false
    color: Style.window
    onReloadChanged: {
        if (!reload) {
            vlHistory.itemCount = logModel.rowCount();
            vlHistory.loadingModel = false;
        } else {
            vlHistory.loadingModel = true;
        }
    }
    Timer {
        id: loadTimer
        interval: 100
        running: false
        onTriggered: {
            console.log("setting itemCount")
            vlHistory.itemCount = logModel.rowCount();
            console.log("loadingModel = false")
            vlHistory.loadingModel = false;
        }
    }
    VirtualListView {
        id: vlHistory
        debug: false
        width: parent.width - 15
        height: parent.height
        viewPosition: historyScrollbar.position
        listMode: "Selection"
        defaultItemHeight: rowHeight
        defaultVliCount: 80
        itemDelegate: historyEntryComponentNew
        onViewMaxPosition: {
            if (logModel.canFetchMore(0)) {
                console.log("VIEWMAX")
                vlHistory.loadingModel = true;
                console.log("load more", logModel.canFetchMore(null))
                logModel.fetchMore(null);
                loadTimer.start();
            }
        }
    }
    ScrollBar2 {
        id: historyScrollbar
        x: parent.width - width
        y: 0
        orientation: Qt.Vertical
        barSize: 15
        height: parent.height
        target: vlHistory
        container: vlHistory
        stepSize: rowHeight
    }
    Component {
        id: historyEntryComponentNew
        Item {
            property int index
            property variant commitId
            property string messageText
            property string summaryText
            property string authorText
            property string timeText
            property variant graphValue
            property bool isMerge
            property int itemHeight: rowHeight
            id: rootItem
            height: rowHeight
            width: vlHistory.width
            clip: true
            function load(idx, elmIdx) {
                if (idx > -1) {
                    var rowData = LibHelper.getCommit(logModel, idx);
                    if (!rowData.cid) {
                        console.log(idx, logModel.rowCount())
                        return
                    }
                    commitId = rowData.cid;
                    messageText = rowData.message;
                    summaryText = rowData.summary;
                    authorText = rowData.author;
                    timeText = rowData.timeHumanized;
                    graphValue = rowData.graph;
                    isMerge = rowData.isMerge;
                }
            }
            function notify() {
                // called when the graph column resizes
                graphComponentRef.requestPaint()
            }
            Item {
                height: parent.height
                width: parent.width
                RowLayout {
                    height: parent.height
                    width: parent.width
                    GraphView {
                        id: graphComponentRef
                        graphHeight: parent.height
                        graphWidth: graphViewSplitter.width
                        graphData: graphValue
                        isMergeNode: isMerge
                        requiresUpdates: true
                    }
                    Rectangle {
                        clip: true
                        Layout.preferredWidth: parent.width - graphViewSplitter.width - rowAuthorRectRef.width - rowTimeRectRef.width - 10
                        Layout.fillHeight: true
                        color: "transparent"
                        TextElement {
                            id: rowMessageRef
                            x: 4 // not sure why, but text can get chopped off otherwise
                            anchors.verticalCenter: parent.verticalCenter
                            text: summaryText
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
                            text: authorText
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
                            text: timeText
                        }
                    }
                }
            }

        }
    }

    Component {
        id: historyEntryComponent
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

    Item {
        id: graphRoot
        width: parent.width
        height: parent.height

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
//            if (historyListView.highlightItem) {
//                var off;
//                if (top) {
//                    off = historyListView.highlightItem.y + rowHeight - historyListView.contentY;
//                    return (off + historyListView.highlightItem.height) < 0 ? historyListView.height : off;
//                } else {
//                    off = historyListView.height - (historyListView.highlightItem.y + historyListView.highlightItem.height - historyListView.contentY);
//                    return off > historyListView.height ? 0 : off;
//                }
//            } else {
//            }
            return top ? vlHistory.height : 0;
        }
        QQC14.SplitView {
            orientation: Qt.Horizontal
            x: 0
            y: 0
            height: graphRoot.getGraphSplitterHeight(true)
            width: parent.width
            handleDelegate: graphSplitterDraggerComponentRef
            Rectangle {
                id: graphViewSplitter
                width: 150
                color: "transparent"
                onWidthChanged: {
                    vlHistory.notify(); // triggers canvas repaints
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
            height: graphRoot.getGraphSplitterHeight(false)
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
                    vlHistory.notify(); // triggers canvas repaints
                    graphViewSplitter.width = width
                }
            }
            Item { }
        }
    }
}
