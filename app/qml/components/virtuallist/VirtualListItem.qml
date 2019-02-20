import QtQuick 2.11
import "../../base"

Rectangle {
    id: root
    // The index of the control in VirtualListView.
    property int vIndex: -1
    // The index of the item being displayed from the model.
    property int itemIndex: itemIndexPlusOne - 1
    property int itemIndexPlusOne: getItemIndex()
    // The offset of the item relative to all previous items.
    property real itemOffset: 0
    property variant itemLineHeights
    property VirtualListShared shared
    onItemIndexChanged: {
        // we get this when we've just been created, and we're getting -1
        if (itemIndex < 0 && !shared) return;
        // lets the component know what to do, either load the index,
        // if index >= 0 or otherwise unload its contents
        if (shared.constantMode) {
            itemOffset = shared.itemOffsets[itemIndex] || 0;
        } else if (shared.selectionMode) {
            itemOffset = shared.selectionModeOffset(itemIndex);
        }
        if (loader.item) {
            loader.item.load(itemIndex, vIndex);
        } else {
            loader.loadIndex = true;
        }
    }
    color: "transparent"
    onHeightChanged: {
        if (height == 0) return; // assume we're reloading or whatever
        if (height !== loader.height) {
            throw new Error('mismatched heights for index', itemIndex, 'expected', height, 'found', loader.height);
        }
    }
    width: parent && parent.width || 0
    visible: getLoaded()
    Loader {
        property bool loadIndex: false
        function getItemHeight() {
            if (shared.constantMode) {
                return shared.itemHeights[itemIndex] || 0;
            } else {
                return shared.defaultItemHeight;
            }
        }
        id: loader
        x: 0
        y: -shared.contentOffset + itemOffset
        height: getItemHeight()
        width: parent && parent.width || 0
        sourceComponent: shared.itemDelegate
        onLoaded: {
            if (loadIndex) {
                item.load(parent.itemIndex, parent.vIndex);
            }
        }
    }
    LayoutHelper {
        x: loader.x
        y: loader.y
        height: loader.height
        width: loader.width
        enabled: shared.debug
        debugText: "elm:" + root.vIndex + ",index:" + root.itemIndex + "\ny:" + (-shared.contentOffset + itemOffset).toFixed(3)
         + "\ncontentOffset:" + (shared.contentOffset.toFixed(3))
    }
    function notify() {
        loader.item.notify();
    }
    function getLoaded() {
        return shared.vlIndex <= shared.vlEnd ?
            shared.vlIndex <= vIndex && vIndex <= shared.vlEnd :
            shared.vlIndex <= vIndex || vIndex <= shared.vlEnd;
    }
    function getItemIndex() {
        if (!shared && vIndex === -1) {
            return -1;
        }
        if (shared.reloading) {
//            if (shared.debug && vIndex === 0) console.log("reload case =>", -1)
            return -1;
        }
        if (shared.updating || !getLoaded()) {
//            if (shared.debug && vIndex === 0) console.log("updating || !loaded case =>", itemIndexPlusOne)
            return itemIndexPlusOne;
        }
        if (shared.vlIndex > shared.vlEnd && vIndex <= shared.vlEnd) {
            var v = shared.itemIndex + (shared.vliCount - shared.vlIndex) + vIndex
//            if (shared.debug) console.log(vIndex, "vindex 1", v)
//            if (shared.debug && vIndex === 0) console.log("branch 1 =>", v)
            return v + 1;
        } else if (shared.vlIndex <= shared.vlEnd || shared.vlIndex <= vIndex) {
            var v = shared.itemIndex + (vIndex - shared.vlIndex)
//            if (shared.debug) console.log(vIndex, "vindex 2", v)
//            if (shared.debug && vIndex === 0) console.log("branch 2 =>", v)
            return v + 1;
        } else {
            throw new Error('unhandled case in getItemIndex')
        }
    }
}
