import QtQuick 2.9
import RustCode 1.0
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Item {
    id: root
    property int rowHeight: 18
    property bool reload: false
    signal diffChanged(string commitOid, int index, string status, string filenameOld, string filenameNew)
    GridView {
        id: diffListViewRef
        // computes an adjusted column width based on the number of chars in a filepath.
        function computeGridWidth() {
            var pw = parent.width - 15; // 15 is for the scrollbar, visible or not
            var max_len = gitModel.diffs.maxFilenameLength * 7 + 15;
            var times = Math.max(Math.floor(pw / max_len), 1);
            var adjusted_len = (pw) / times;
            return isNaN(adjusted_len) ? 0 : adjusted_len;
        }
        property bool reloadFirst: true
        property int gridItemWidth: computeGridWidth()
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        anchors.right: diffScrollRef.left
        currentIndex: 0
        // todo: must be a better way to detect list has been reloaded/rebound.
        // this relies on the fact that currentIndex fires before currentItem,
        // but at least this resets the selected row when changing commits.
        property bool indexChanged: false
        onCurrentIndexChanged: {
            indexChanged = true
        }
        onCurrentItemChanged: {
            if (!indexChanged) {
                currentIndex = 0
            } else {
                indexChanged = false
            }
            if (reload && !reloadFirst) {
                reload = false;
            } else {
                if (reloadFirst) {
                    reload = reloadFirst = false;
                }
                root.diffChanged(
                    gitModel.diffs.commitOid,
                    currentIndex,
                    currentItem.statusText,
                    currentItem.filenameOldText,
                    currentItem.filenameNewText
                );
            }
        }
        clip: true
        cellHeight: rowHeight
        cellWidth: gridItemWidth
        model: gitModel.diffs
        interactive: false
        highlightMoveDuration: 1
        keyNavigationEnabled: true
        highlightFollowsCurrentItem: true
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
        ScrollBar.vertical: ScrollBar {
            id: diffRealScrollRef
            width: 0
        }
        delegate: Component {
            Item {
                height: rowHeight
                property string filenameOldText: filenameOld
                property string filenameNewText: filenameNew
                property string statusText: status
                width: diffListViewRef.gridItemWidth
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
                    onWheel: {
                        var isDown = wheel.angleDelta.y < 0;
                        var topIdx = Math.max(0, diffListViewRef.indexAt(1, diffListViewRef.contentY + 1) + 3 * (isDown ? 1 : -1));
                        diffListViewRef.positionViewAtIndex(topIdx, ListView.Beginning);
                    }
                }
            }
        }
    }

    DesktopScrollbar {
        id: diffScrollRef
        anchors.top: parent.top
        anchors.right: parent.right
        scrollHeight: parent.height
        scrollSize: diffRealScrollRef.size
        scrollPosition: diffRealScrollRef.position
        onPositionChanged: {
            diffRealScrollRef.position = position
        }
        onStep: {
            var stepVal = down ? 1 : -1;
            var topIdx = Math.max(0, diffListViewRef.indexAt(1, diffListViewRef.contentY + 1) + stepVal);
            diffListViewRef.positionViewAtIndex(topIdx, ListView.Beginning);
        }
    }
}
