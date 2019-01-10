import QtQuick 2.11
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"
import "../scripts/utils.js" as Utils

Rectangle {
    // The hunk listings contains alot of state, to make it usable, chiefly:
    // 1. The hunk id property is used to detect "resetting", when set as "",
    //    which happens in GitView.qml. This is done to limit the number of
    //    times we might otherwise recompute the height of the listview.
    // 2. With variable height listview items, alot of additional stuff needs
    //    to be handled by ourselves.
    // 3. The floating horizontal scrollbar is somewhat heavy handed as well,
    //    as whenever the size of the container, or the listview is scrolled,
    //    we need to requery for what items is currently at the bottom
    // 4. All of compounded by a variable height header!
    id: root
    color: "transparent"
    property string filenameOld: ""
    property string filenameNew: ""
    property string statusText: ""
    // commitsha + idx, "x" as default value to detect when GitView
    // initially resets the model for the first diff.
    property string hunkId: "x"
    property variant originList
    property int floatScrollBarIndex: -1;
    property real floatScrollBarOffset: 0;
    property int hunkItemLineCount: 0
    property real listContentHeight: 0
    Keys.forwardTo: [hunksMainScrollRef]
    //onHeightChanged: floatScrollBarIndex = getBottomElementIndex()
    //onWidthChanged: floatScrollBarIndex = getBottomElementIndex()
    onHunkIdChanged: {
        //console.log("hunkid changed", hunkId, hunkListViewRef.loadingModel)
        if (!hunkId) {
            hunkListViewRef.loadingModel = true;
            listContentHeight = 0;
            hunksMainScrollRef.position = 0;
        } else {
            hunkListViewRef.loadingModel = false;
        }
        //floatScrollBarIndex = getBottomElementIndex()
        //listContentHeight = getHeight();
        //hunkListViewRef.height = listContentHeight;
    }
    function getBottomElementIndex() {
        // 4.5 is fudged value to mark as "bottom" when
        // the scrollbar crosses the window bottom.
        var scrollOffset = -hunkListViewRef.y;
        var offset = -hunkListViewRef.y + (hunkListViewRectContainerRef.height + 4.5);
        floatScrollBarOffset = offset - 20
        return hunkListViewRef.indexAt(0, offset);
    }
    function getHeight() {
        var h = 0;
        var itemOtherH = 55;
        var itemLineH = Style.fontFixedLineHeight;
        hunkItemLineCount = 0;
        for (var i = 0; i < gitModel.hunks.rowCount(); i++) {
            var linesOrigin = LibHelper.modelValue(gitModel.hunks, i, LibHelper.hunks_linesOrigin);
            var ba = new Uint8Array(linesOrigin);
            hunkItemLineCount += ba.length;
            h += ba.length * itemLineH;
            h += itemOtherH;
        }
        return h;
    }
    Rectangle {
        property bool isComparison: filenameOld !== filenameNew
        id: headerRectRef
        height: 3 + (isComparison ? (fnBotRef.y + fnBotRef.contentHeight)
                                  : (fnTopRef.y + fnTopRef.contentHeight))
        width: parent.width
        anchors.top: parent.top
        anchors.left: parent.left
        clip: true
        color: "transparent"
        DiffStatusIcon {
            y: 1
            x: 0
            statusValue: headerRectRef.isComparison ? "Deleted" : statusText
            iconSize: 13
        }
        DiffStatusIcon {
            x: 0
            y: fnTopRef.y + fnTopRef.contentHeight
            visible: headerRectRef.isComparison
            statusValue: "Added"
            iconSize: 13
        }
        TextElement {
            id: fnTopRef
            x: 20
            y: 4
            width: parent.width - 30
            wrapMode: Text.WrapAnywhere
            text: headerRectRef.isComparison ? filenameOld : filenameNew
        }
        TextElement {
            id: fnBotRef
            x: 20
            y: fnTopRef.y + fnTopRef.contentHeight + 3
            visible: headerRectRef.isComparison
            width: parent.width - 30
            wrapMode: Text.WrapAnywhere
            text: filenameNew
        }
    }
    Rectangle {
        id: headerRectBorderRef
        anchors.top: headerRectRef.bottom
        width: parent.width
        height: 1
        color: Style.mid
        visible: false
    }
    Rectangle {
        // The hunkListViewRef listview contains variable height elements. Even
        // with just a few elements in the list, Qt will compute the full height,
        // based on the elements loaded in the view (a slice of the full list).
        // since these are variable, the contentHeight of the container fluxes
        // and cannot be used for predictable layout. Instead we compute the
        // approximate height of the full list (based on the number of lines in
        // each hunk), and scroll by offsetting the y coordinate of the listview.
        // This gives a nice stable scrollbar that doesn't twitch when dragged.
        id: hunkListViewRectContainerRef
        anchors.top: headerRectBorderRef.bottom
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        color: "transparent"
        clip: true
        VirtualListView {
            id: hunkListViewRef
            width: parent.width - hunksMainScrollRef.width
            height: parent.height
            debug: false
            viewPosition: hunksMainScrollRef.position
            textContentColumn: LibHelper.hunks_hunk
            heightColumn: LibHelper.hunks_lines
            heightValueFactor: Style.fontFixedLineHeight
            extraItemHeight: 20
            items: ListView { model: gitModel.hunks; delegate: Component { Item { } } }
            metricsHelper: Item {
                function get() {
                    hunkLineCache.init(gitModel.hunks.hunkListings, gitModel.hunks.rowCount());
                    var h = 0;
                    var ih;
                    var newHeights = [];
                    var newOffsets = [];
                    var newLineHeights = [];
                    var ts = Utils.now();
                    for(var i = 0; i < gitModel.hunks.rowCount(); i++) {
                        var content = LibHelper.modelValue(gitModel.hunks, i, LibHelper.hunks_hunk);
                        var listModel = hunkLineCache.get(i);
                        var txtDims = Style.getTextDims(content, true, true);
                        hunkLineCache.updateLines(i, txtDims.lineHeights);
                        ih = txtDims.height + hunkListViewRef.extraItemHeight;
                        newHeights.push(ih);
                        newOffsets.push(h);
                        newLineHeights.push(listModel);
                        h += ih;
                    }
                    console.log("metrics duration", Utils.duration(ts))
                    return {
                        contentHeight: h,
                        heights: newHeights,
                        offsets: newOffsets,
                        lineHeights: newLineHeights
                    };
                }
            }
            itemDelegate: hunkTemplate
        }
        CustomScrollBar {
            id: hunksMainScrollRef
            x: parent.width - width
            y: 0
            width: size < 1 ? 15 : 0
            height: parent.height
            orientation: Qt.Vertical
            size: height / hunkListViewRef.contentHeight
            stepSize: 1 / (hunkListViewRef.contentHeight / (Style.fontFixedLineHeight * 2))
            pageScrollOverlapSize: Style.fontFixedLineHeight * 2
            scrollContainerSize: height
            scrollContentSize: hunkListViewRef.contentHeight
            captureMouseWheel: true
            capturePositiveSide: false
            containerOtherSize: parent.width
        }
    }
    // Todo some nicer way of handling focus
    MouseArea {
        anchors.fill: parent;
        onPressed: {
            mouse.accepted = false;
            root.forceActiveFocus();
        }
    }
    Component {
        id: listModelRef
        ListModel { }
    }
    HunkLineModelCache {
        id: hunkLineCache
        onLinesModelReady: hunkListViewRef.notify(index)
    }
    Component {
        id: hunkTemplate
        Item {
            id: hunkComp
            height: hunkTitleRectRef.height + compTxt.contentHeight
            width: hunkListViewRef.width
            property int index
            property int linesTo
            property int linesFrom
            property ListModel lineHeights
            function load(elmId, idx) {
                if (idx > -1) {
                    var txt = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_hunk);
                    if (!txt) {
                        throw new Error('undefined text for ', idx)
                    }
                    linesFrom = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_linesFrom);
                    linesTo = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_linesTo);
                    var linesData = hunkLineCache.get(idx);
                    lineHeights = linesData.list;
                    if (linesData.ready) {
                        notify()
                    }
                    compTxt.text = txt;
                    index = idx;
                } else {
                    compTxt.text = '';
                }
            }
            function notify() {
                lineCanvas.render = true;
                lineCanvas.requestPaint();
            }
            Rectangle {
                id: hunkTitleRectRef
                anchors.top: parent.top
                anchors.left: parent.left
                width: parent.width
                height: 20
                color: "transparent"
                TextElement {
                    property bool decodeError: linesTo === 0 && linesFrom === 0
                    function getLineText() {
                        return decodeError ? " : failed to decode hunk as UTF-8"
                                           : " : " + (linesTo - linesFrom + 1) + " lines";
                    }
                    x: 5
                    y: 4
                    opacity: 0.6
                    color: decodeError ? "red" : "black"
                    text: "Hunk " + (index + 1) + getLineText()
                }
            }
            Canvas {
                id: lineCanvas
                x: 0
                y: 20
                height: compTxt.contentHeight
                width: parent.width
                property bool render: false
                onPaint: {
                    if (render && lineHeights) {
                        render = false;
                        var y = 0;
                        var ctx = getContext("2d");
                        var ts = Utils.now();
                        for(var i = 0; i < lineHeights.count; i++) {
                            var l = hunkComp.lineHeights.get(i);
                            ctx.fillStyle = Style.lineOriginColor(l.origin);// === 10 ? 'white' : Qt.rgba(1,0,0,0.5);
                            ctx.fillRect(0, y, width, l.height);
                            y += l.height;
                        }
                        console.log("CANVAS, line count", lineHeights.count, 'rendering took', Utils.duration(ts))
                    }
                }
            }
            TextElement {
                x: 0
                y: 20
                id: compTxt
                width: parent.width
                selectableText: true
                fixedWidthFont: true
            }
        }
    }
}
