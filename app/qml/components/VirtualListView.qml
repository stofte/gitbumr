import QtQuick 2.11
import "../base"
import "../style"

Rectangle {
    id: root
    width: parent.width
    height: parent.height
    color: "transparent"
    property bool debug: false
    property real preloadSize: 100
    property real contentHeight: shared.contentHeight
    property int textContentColumn
    property int heightColumn
    property int heightValueFactor: 1
    property int extraItemHeight: 0
    property real viewPosition: 0
    property bool loadingModel: true
    property ListView items
    property Component itemDelegate
    property variant virtualItems
    property VirtualListShared shared: VirtualListShared {
        debug: root.debug
        itemOffsets: []
        itemHeights: []
        itemLineHeights: []
        vliCount: 10
        itemDelegate: root.itemDelegate
        contentOffset: root.viewPosition * contentHeight
        property real prevOffset
        onContentOffsetChanged: {
            root.update(contentOffset, prevOffset < contentOffset, false);
            prevOffset = contentOffset;
        }
    }
    onLoadingModelChanged: {
        if (!loadingModel) {
            update(0, false, true);
            shared.reloading = false;
        } else {
            shared.reloading = true;
        }
    }
    onHeightChanged: {
        update(shared.contentOffset, false, false)
    }
    function getMetrics() {
        var h = 0;
        var ih;
        var newHeights = [];
        var newOffsets = [];
        var newLineHeights = [];
        for(var i = 0; i < items.model.rowCount(); i++) {
            var content = LibHelper.modelValue(items.model, i, textContentColumn);
            var listModel = listModelRef.createObject(null);
            var txtDims = Style.getTextDims(content, true, listModel);
            ih = txtDims.height + extraItemHeight;
            newHeights.push(ih);
            newOffsets.push(h);
            newLineHeights.push(listModel);
            h += ih;
        }
        return {
            contentHeight: h,
            heights: newHeights,
            offsets: newOffsets,
            lineHeights: newLineHeights
        };
    }
    function update(offset, isIncrease, includeMetrics) {
        shared.itemCount = items.model.rowCount();
        if (shared.itemCount === 0) {
            shared.itemIndex = 0;
            return;
        }
        shared.updating = true;
        if (includeMetrics) {
            var metrics = getMetrics();
            shared.contentHeight = metrics.contentHeight;
            shared.itemHeights = metrics.heights;
            shared.itemOffsets = metrics.offsets;
            shared.itemLineHeights = metrics.lineHeights;
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
            throw new Error('too few virtuel items');
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
        shared.itemIndex = fromIdx;
        shared.vlIndex = vlIndex;
        shared.vlEnd = vlEnd;
        shared.updating = false;
//        if (wrote) console.log("INDEX updated");
    }
    Component {
        id: listModelRef
        ListModel { }
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
