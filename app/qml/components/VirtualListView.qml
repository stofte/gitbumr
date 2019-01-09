import QtQuick 2.11
import "../base"

Rectangle {
    id: root
    width: parent.width
    height: parent.height
    color: "transparent"
    property bool debug: false
    property real preloadSize: 100
    property real contentHeight: shared.contentHeight
    property int heightColumn
    property int heightValueFactor: 1
    property real viewPosition: 0
    property bool loadingModel: true
    property ListView items
    property Component itemDelegate
    property variant virtualItems
    property VirtualListShared shared: VirtualListShared {
        debug: root.debug
        itemOffsets: []
        itemHeights: []
        vliCount: 10
        itemDelegate: root.itemDelegate
        contentOffset: root.viewPosition * contentHeight
        property real prevOffset
        onContentOffsetChanged: {
            root.updateIndexes(contentOffset, prevOffset < contentOffset);
            prevOffset = contentOffset;
        }
    }
    onLoadingModelChanged: {
        if (!loadingModel) {
            var i;
            // iterate model and get heights of each element ...
            var h = 0;
            for(i = 0; i < items.model.rowCount(); i++) {
                var ih = LibHelper.modelValue(items.model, i, heightColumn) * heightValueFactor;
                shared.itemHeights.push(ih);
                shared.itemOffsets.push(h);
                h += ih;
            }
            shared.contentHeight = h;
            updateIndexes(0, false);
            shared.reloading = false;
        } else {
            shared.itemHeights = [];
            shared.itemOffsets = [];
            shared.reloading = true;
        }
    }
    onHeightChanged: {
        updateIndexes(shared.contentOffset, false)
    }
    function updateIndexes(offset, isIncrease) {
        shared.itemCount = items.model.rowCount();
        if (shared.itemCount === 0) {
            shared.itemIndex = 0;
            return;
        }
        var fromIdx = -1;
        var toIdx = -1;
        var adjustedOffset = Math.max(offset - 100, 0);
        var toOffset = Math.min(offset + height + preloadSize, shared.contentHeight);
        var sum = 0;
        for (var i = 0; i < items.model.rowCount(); i++) {
            var newSum = sum + shared.itemHeights[i];
            if (fromIdx === -1 && newSum >= adjustedOffset) {
                fromIdx = i;
            }
            if (toIdx === -1 && newSum >= toOffset) {
                toIdx = i;
                break;
            }
            sum += shared.itemHeights[i];
        }
        var loadCount = toIdx - fromIdx + 1;
        if (loadCount > shared.vliCount) {
            throw new Error('too few virtuel items')
        }
        var idxDiff = (fromIdx - shared.itemIndex) % shared.vliCount;
        var vlIndex = (shared.vlIndex + idxDiff) % shared.vliCount;
        var vlEnd = (shared.vlIndex + idxDiff + loadCount - 1) % shared.vliCount;
        if (vlIndex < 0) {
            vlIndex = shared.vliCount + vlIndex;
        }
        if (vlEnd < 0) {
            vlEnd = shared.vliCount + vlEnd;
        }
        var wrote = shared.vlIndex !== vlIndex || shared.vlEnd !== vlEnd || shared.itemIndex !== fromIdx;
//        if (wrote) console.log("INDEX", fromIdx, "count", loadCount, "\t", vlIndex, "to", vlEnd);
        shared.updating = true;
        shared.itemIndex = fromIdx;
        shared.vlIndex = vlIndex;
        shared.vlEnd = vlEnd;
        shared.updating = false;
//        if (wrote) console.log("INDEX updated");
    }
    Component.onCompleted: {
        root.virtualItems = [];
        var cmp = Qt.createComponent("VirtualListItem.qml");
        for (var i = 0; i < shared.vliCount; i++) {
            var elm = cmp.createObject(root)
            elm.vIndex = i;
            elm.shared = root.shared;
            root.virtualItems.push(elm);
            root.children.push(elm);
        }
    }
}
