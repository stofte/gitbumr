import QtQuick 2.11
import "../base"

Rectangle {
    id: root
    // The index of the control in VirtualListView.
    property int vIndex
    // The index of the item being displayed from the model.
    property int itemIndex: getItemIndex()
    // The offset of the item relative to all previous items.
    property real itemOffset: 0
    property variant itemLineHeights
    property VirtualListShared shared
    onItemIndexChanged: {
        // lets the component know what to do, either load the index,
        // if index >= 0 or otherwise unload its contents
        itemOffset = shared.itemOffsets[itemIndex] || 0;
        itemLineHeights = shared.itemLineHeights[itemIndex];
        loader.item.load(vIndex, itemIndex, itemLineHeights);
    }
    color: "transparent"
    y: -shared.contentOffset + itemOffset
    height: shared.itemHeights[itemIndex] || 0
    onHeightChanged: {
        if (height == 0) return; // assume we're reloading or whatever
        if (height !== loader.height) {
            throw new Error('mismatched heights for index', itemIndex, 'expected', height, 'found', loader.height);
        }
    }
    width: parent && parent.width || 0
    visible: getLoaded()
    LayoutHelper {
        height: parent.height
        width: parent.width
        enabled: shared.debug
        debugText: "elm:" + root.vIndex + ",index:" + root.itemIndex + "\ny:" + (-shared.contentOffset + itemOffset).toFixed(3)
        Loader {
            id: loader
            sourceComponent: shared.itemDelegate
        }
    }
    function getLoaded() {
        return shared.vlIndex <= shared.vlEnd ?
            shared.vlIndex <= vIndex && vIndex <= shared.vlEnd :
            shared.vlIndex <= vIndex || vIndex <= shared.vlEnd;
    }
    function getItemIndex() {
        if (shared.reloading) {
            return -1;
        }
        if (shared.updating || !getLoaded()) {
            return itemIndex;
        }
        if (shared.vlIndex > shared.vlEnd && vIndex <= shared.vlEnd) {
            return shared.itemIndex + (shared.vliCount - shared.vlIndex) + vIndex
        } else if (shared.vlIndex <= shared.vlEnd || shared.vlIndex <= vIndex) {
            return shared.itemIndex + (vIndex - shared.vlIndex)
        } else {
            throw new Error('unhandled case in getItemIndex')
        }
    }
}
