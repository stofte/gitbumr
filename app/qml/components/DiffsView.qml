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
    signal diffChanged(string commitOid, int index)
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
        function mapGitStatusToColor(status) {
            switch (status) {
                case "Modified":
                    return "#F6C342";
                case "Added":
                    return "#14892C";
                case "Deleted":
                    return "#D04437";
                case "Renamed":
                    return "#AC707A";
                default:
                    throw new Error('mapGitStatusToColor hnhandled git status: ' + status);
            }
        }
        function mapGitStatusToLetterOffset(status) {
            switch (status) {
                case "Modified":
                    return 3;
                case "Renamed":
                case "Deleted":
                case "Added":
                    return 4;
                default:
                    throw new Error('mapGitStatusToLetterOffset unhandled git status: ' + status);
            }
        }
        function mapGitStatusToLetter(status) {
            return status.substring(0, 1);
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
                root.diffChanged(gitModel.diffs.commitOid, currentIndex);
            }
        }
        clip: true
        cellHeight: rowHeight
        cellWidth: gridItemWidth
        model: gitModel.diffs
        interactive: false
        //highlightResizeDuration: 1
        highlightMoveDuration: 1
        //highlightMoveVelocity: -1
        keyNavigationEnabled: true
        highlightFollowsCurrentItem: true
        highlight: Component{
            Item {
                function getColor() {
                    return diffListViewRef.currentItem ? diffListViewRef.mapGitStatusToColor(diffListViewRef.currentItem.statusText)
                                                       : "#000000";
                }
                function getLetter() {
                    return diffListViewRef.currentItem ? diffListViewRef.mapGitStatusToLetter(diffListViewRef.currentItem.statusText)
                                                       : "";
                }
                function getLetterOffset() {
                    return diffListViewRef.currentItem ? diffListViewRef.mapGitStatusToLetterOffset(diffListViewRef.currentItem.statusText)
                                                       : 0;
                }

                z: 2
                height: rowHeight
                clip: true
                Rectangle{
                    anchors.fill: parent
                    color: Style.selection
                    Rectangle {
                        height: rowHeight - 5
                        width: rowHeight - 5
                        x: 3
                        y: 2.5
                        color: getColor()
                        TextItem {
                            x: getLetterOffset()
                            y: 1
                            color: "white"
                            font.weight: Font.Bold
                            text: getLetter()
                        }
                    }
                    TextItem {
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
                property string filenameNewText: filenameNew
                property string statusText: status
                width: diffListViewRef.gridItemWidth
                clip: true
                Rectangle {
                    height: rowHeight - 5
                    width: rowHeight - 5
                    x: 3
                    y: 2.5
                    color: diffListViewRef.mapGitStatusToColor(status)

                    TextItem {
                        x: diffListViewRef.mapGitStatusToLetterOffset(status)
                        y: 1
                        color: "white"
                        font.weight: Font.Bold
                        text: diffListViewRef.mapGitStatusToLetter(status)
                    }
                }
                TextItem {
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
