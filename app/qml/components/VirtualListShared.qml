import QtQuick 2.0

Item {
    property bool debug
    property real contentOffset
    property variant itemOffsets
    property variant itemHeights
    property real contentHeight
    property Component itemDelegate
    // used in VirtualListItem to guard against partially updated values.
    property bool updating
    // used in VirtualListItem to reset model id (returning -1 on reload).
    property bool reloading: true
    // the number of instantiated virtuallist items. not all might be required
    // for a given hunk viewport, and no more must be required for layout.
    property int vliCount
    // vlIndex is where the list splits between front/end, meaning that
    // vlIndex == id is at the front/top of the virtual list, and components
    // after it are loaded after vlIndex, if theres enough items and
    // the component is required because its within the viewport range.
    property int vlIndex: 0
    // the index for the list item that should be the last item loaded.
    property int vlEnd: 0
    // index into the item model
    property int itemIndex: 0
    // number of items in the model
    property int itemCount: 0
    // the index which crosses the bottom
    property int itemIndexBottom
    property real bottomBorderLocalOffset
    function getVirtualListActiveCount() {
        if (vlIndex <= vlEnd) {
            return vlEnd - vlIndex + 1;
        } else {
            return vliCount - vlIndex + vlEnd + 1;
        }
    }
}
