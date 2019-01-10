import QtQuick 2.9
import RustCode 1.0
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Rectangle {
    id: root
    property int rowHeight: 18
    property int scrollBarWidth: 15
    property string commitId: ""
    property real computedCellWidth: 0
    property variant textWidths: []
    signal diffChanged(string commitOid, int index, string status, string filenameOld, string filenameNew)
    function updateGridDimensions() {
        var gridItemWidth = 0;
        var ws = textWidths;
        var rowCount =  gitModel.diffs.rowCount();
        if (ws.length !== rowCount) {
            // ideally, we only do this once in "onCommitIdChanged"
            for (var i = 0; i < gitModel.diffs.rowCount(); i++) {
                var filename = LibHelper.modelValue(gitModel.diffs, i, LibHelper.diffs_filenameNew);
                var fnWidth = Style.getTextDims(filename).width;
                ws.push(fnWidth);
            }
            ws = ws.sort(function (a, b) { return  a - b }); // assumes all numbers
        }
        if (ws.length > 0) {
            var rootWidth = root.width;
            var rootHeight = Math.max(root.height - 15, root.rowHeight);
            // grab the largest width, could do something where we cut off some items
            // if that yields to more columns, etc.
            var itemMaxSize = ws[ws.length - 1] + 25; // icon space + padding
            var ratio = rootWidth / itemMaxSize;
            var columnsRequiredNoScroll = Math.ceil(rowCount / Math.floor(rootHeight / root.rowHeight));
            var expectedOverflow = columnsRequiredNoScroll * itemMaxSize > rootWidth;
            if (expectedOverflow) {
                gridItemWidth = itemMaxSize
            } else {
                gridItemWidth = rootWidth / columnsRequiredNoScroll
            }
        }
        textWidths = ws;
        computedCellWidth = gridItemWidth;
    }
    onCommitIdChanged: {
        textWidths = [];
        diffListViewRef.currentIndex = 0;
        updateGridDimensions();
    }
    onHeightChanged: updateGridDimensions()
    onWidthChanged: updateGridDimensions()
    color: "transparent"
    clip: true
    height: parent.heigt
    LayoutHelper {
        height: parent.height
        width: parent.width
        enabled: false
        GridView {
            id: diffListViewRef
            x: 0
            currentIndex: 0
            width: parent.width
            height: parent.height - 15
            cellHeight: rowHeight
            cellWidth: computedCellWidth
            model: gitModel.diffs
            flow: GridView.FlowTopToBottom
            interactive: false
            keyNavigationEnabled: true
            highlightMoveDuration: 1
            highlightFollowsCurrentItem: true
            onCurrentItemChanged: {
                if (currentItem) {
                    // todo: yields a few extra signals when switching list contents, etc
                    root.diffChanged(
                        gitModel.diffs.commitOid,
                        currentIndex,
                        currentItem.statusText,
                        currentItem.filenameOldText,
                        currentItem.filenameNewText
                    );
                }
            }
            Keys.onPressed: {
                if (event.key === Qt.Key_PageDown) {
                    console.log("page down");
                } else {
                    event.accepted = false;
                }
            }
            ScrollBar.horizontal: CustomScrollBar {
                id: gridScrollRef
                transform: Translate { y: 15 }
                policy: ScrollBar.AlwaysOn
                orientation: Qt.Horizontal
                adjustPositionOnResize: false
                height: 15
                enabled: size < 1
                stepSize: root.rowHeight / diffListViewRef.contentWidth
                captureMouseWheel: true
                capturePositiveSide: true
                containerOtherSize: parent.height + 15
                scrollContainerSize: parent.width
                scrollContentSize: diffListViewRef.contentWidth
                // custom page step according to cell width
                pageScrollStepSize: computedCellWidth / diffListViewRef.contentWidth
            }
            highlight: Component{
                Item {
                    z: 2
                    height: rowHeight
                    clip: true
                    Rectangle{
                        anchors.fill: parent
                        color: Style.selection
                        DiffStatusIcon {
                            statusValue: diffListViewRef.currentItem ? diffListViewRef.currentItem.statusText : ""
                            iconSize: rowHeight - 5
                        }
                        TextElement {
                            x: 20
                            y: 3
                            color: "white"
                            text: diffListViewRef.currentItem && diffListViewRef.currentItem.filenameNewText
                        }
                    }
                }
            }
            delegate: Component {
                Rectangle {
                    color: "transparent"
                    height: rowHeight
                    property string filenameOldText: filenameOld
                    property string filenameNewText: filenameNew
                    property string statusText: status
                    width: root.computedCellWidth
                    clip: true
                    DiffStatusIcon {
                        statusValue: status
                        iconSize: rowHeight - 5
                    }
                    TextElement {
                        x: 20
                        y: 3
                        text: filenameNewText
                    }
                    MouseArea {
                        anchors.fill: parent
                        onPressed: {
                            diffListViewRef.currentIndex = index;
                            diffListViewRef.forceActiveFocus();
                        }
                    }
                }
            }
        }
    }

}
