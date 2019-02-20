import QtQuick 2.11
import "../../base"
import "../../style"

Rectangle {
    id: root
    width: parent.width
    height: parent.height
    color: "transparent"
    property string listMode: "Constant"
    readonly property int constantMode: listMode === "Constant"
    readonly property int selectionMode: listMode === "Selection"
    property bool debug: false
    property real preloadSize: 100
    property real contentHeight: shared.contentHeight
    property real contentOffset: shared.contentOffset
    // the item should only be marked as "bottom index"
    // only within these offsets
    property real bottomIndexTopOffset
    property int indexBottom: shared.itemIndexBottom
    property real bottomBorderLocalOffset
    property int textContentColumn
    property int heightColumn
    property int heightValueFactor: 1
    property int extraItemHeight: 0
    property real viewPosition: 0
    onViewPositionChanged: {
        console.log("vlv viewPosition", viewPosition)
    }
    property bool loadingModel: true
    property ListView items
    property int itemCount: 0
    property Component itemDelegate
    property Item metricsHelper
    property variant virtualItems
    // for selectionMode only
    property real selectionItemHeight
    property int currentIndex
    property real defaultItemHeight
    property int defaultVliCount: 10
    signal viewMaxPosition();
    function notify(index) {
        if (index) {
            var vi = getVirtualItem(index);
            if (vi) {
                vi.notify();
            }
        } else if (virtualItems) {
            for (var i = 0; i < virtualItems.length; i++) {
                virtualItems[i].notify();
            }
        }
    }
    function getVirtualItem(index) {
        if (shared.itemIndex <= index) {
            var c = shared.getVirtualListActiveCount();
            var idxDiff = index - shared.itemIndex;
            if (idxDiff < c) {
                if (shared.vlIndex <= shared.vlEnd || shared.vlIndex + idxDiff < shared.vliCount) {
                    return virtualItems[shared.vlIndex + idxDiff];
                } else {
                    idxDiff = idxDiff - (shared.vliCount - shared.vlIndex);
                    return virtualItems[idxDiff];
                }
            }
        }
        return null;
    }
    property bool fullyLoaded: false
    property bool checkForMore: true
    property bool disableViewPosition: false
    property VirtualListShared shared: VirtualListShared {
        debug: root.debug
        itemOffsets: []
        itemHeights: []
        vliCount: root.defaultVliCount
        itemDelegate: root.itemDelegate
        contentOffset: root.viewPosition * contentHeight
        constantMode: root.constantMode
        selectionMode: root.selectionMode
        defaultItemHeight: root.defaultItemHeight
        disableViewPosition: root.disableViewPosition
        property real prevOffset
        onContentOffsetChanged: {
            if (constantMode) {
                root.updateConstantMode(contentOffset, prevOffset < contentOffset, false);
            } else {
                root.updateSelectionMode(contentOffset, prevOffset < contentOffset);
            }
            if (contentOffset + root.height >= contentHeight) {
                root.viewMaxPosition();
            }
            prevOffset = contentOffset;
        }
    }
    onLoadingModelChanged: {
        if (debug) {
            console.log("loadingModel", loadingModel)
        }
        if (!loadingModel) {
            if (constantMode) {
                updateConstantMode(0, false, true);
            } else {
                updateSelectionMode(0, false);
            }
        }
        shared.reloading = loadingModel;
    }
    onHeightChanged: {
        if (constantMode) {
            updateConstantMode(shared.contentOffset, false, false)
        } else {
            updateSelectionMode(shared.contentOffset, false);
        }
    }
    function updateSelectionMode(offset, isIncrease) {
//        console.log(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\nupdateSelectionMode")
        shared.itemCount = itemCount;
        if (shared.itemCount === 0) {
            shared.itemIndex = 0;
            return;
        }
        shared.updating = true;
        // assume always selected
        shared.contentHeight = (shared.itemCount) * defaultItemHeight;
//        var fromIdx = -1;
//        var toIdx = -1;
        var fromIdx = Math.floor(Math.max(offset - 100, 0) / defaultItemHeight);
        var toIdx = Math.min(fromIdx + Math.ceil((height + preloadSize) / defaultItemHeight), shared.itemCount - 1);
        var vIdxs = getVirtualIndexes(fromIdx, toIdx);
        shared.itemIndex = fromIdx;
        shared.vlIndex = vIdxs[0];
        shared.vlEnd = vIdxs[1];
        if (debug) console.log("vlv", fromIdx, offset, JSON.stringify(vIdxs))
        shared.updating = false;
    }
    function updateConstantMode(offset, isIncrease, includeMetrics) {
        shared.itemCount = itemCount;
        if (shared.itemCount === 0) {
            shared.itemIndex = 0;
            return;
        }
        shared.updating = true;
        if (includeMetrics) {
            var metrics = metricsHelper.get(); // getMetrics();
            shared.contentHeight = metrics.contentHeight;
            shared.itemHeights = metrics.heights;
            shared.itemOffsets = metrics.offsets;
        }
        var fromIdx = -1;
        var toIdx = -1;
        var bottomIdx = -1;
        var adjustedOffset = Math.max(offset - 100, 0);
        var toOffset = Math.min(offset + height + preloadSize, shared.contentHeight);
        var bottomCrossFrom = offset + height - bottomIndexTopOffset;
        var bottomCrossTo = offset + height;
        var sum = 0;
        for (var i = 0; i < itemCount; i++) {
            var newSum = sum + shared.itemHeights[i];
            if (fromIdx === -1 && newSum >= adjustedOffset) {
                fromIdx = i;
            }
            if (bottomIdx === -1 && newSum >= bottomCrossFrom) {
                if (newSum >= bottomCrossTo) {
                    bottomIdx = i;
                } else {
                    bottomIdx = -2;
                }
            }
            if (toIdx === -1 && newSum >= toOffset) {
                toIdx = i;
                break;
            }
            sum += shared.itemHeights[i];
        }
        var vIdxs = getVirtualIndexes(fromIdx, toIdx);
//        var loadCount = toIdx - fromIdx + 1;
//        if (loadCount > shared.vliCount) {
//            throw new Error('too few virtuel items');
//        }
//        var idxDiff = (fromIdx - shared.itemIndex) % shared.vliCount;
//        var vlIndex = (shared.vlIndex + idxDiff) % shared.vliCount;
//        var vlEnd = (shared.vlIndex + idxDiff + loadCount - 1) % shared.vliCount;
//        if (vlIndex < 0) {
//            vlIndex = shared.vliCount + vlIndex;
//        }
//        if (vlEnd < 0) {
//            vlEnd = shared.vliCount + vlEnd;
//        }
        var wrote = shared.vlIndex !== vIdxs[0] || shared.vlEnd !== vIdxs[1] || shared.itemIndex !== fromIdx;
//        if (wrote) console.log("INDEX", fromIdx, "count", loadCount, "\t", vlIndex, "to", vlEnd);
        shared.itemIndexBottom = bottomIdx;
        shared.itemIndex = fromIdx;
        shared.vlIndex = vIdxs[0];
        shared.vlEnd = vIdxs[1];
//        console.log("bottomIndex", bottomIdx)
        shared.updating = false;
//        if (wrote) console.log("INDEX updated");
    }
    function getVirtualIndexes(from, to) {
        var loadCount = to - from + 1;
        if (loadCount > shared.vliCount) {
            throw new Error('too few virtuel items');
        }
        var idxDiff = (from - shared.itemIndex) % shared.vliCount;
        var vlIndex = (shared.vlIndex + idxDiff) % shared.vliCount;
        var vlEnd = (shared.vlIndex + idxDiff + loadCount - 1) % shared.vliCount;
        if (vlIndex < 0) {
            vlIndex = shared.vliCount + vlIndex;
        }
        if (vlEnd < 0) {
            vlEnd = shared.vliCount + vlEnd;
        }
        return [
            vlIndex,
            vlEnd
        ];
    }

    Component {
        id: listModelRef
        ListModel { }
    }
    Component.onCompleted: {
        root.virtualItems = [];
        var cmp = Qt.createComponent("VirtualListItem.qml");
        for (var i = 0; i < shared.vliCount; i++) {
            var elm = cmp.createObject(root);
            elm.shared = root.shared;
            elm.vIndex = i;
            root.virtualItems.push(elm);
            root.children.push(elm);
        }
    }
}
