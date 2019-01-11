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
    onHunkIdChanged: {
        //console.log("hunkid changed", hunkId, hunkListViewRef.loadingModel)
        if (!hunkId) {
            hunkListViewRef.loadingModel = true;
            listContentHeight = 0;
            hunksMainScrollRef.position = 0;
        } else {
            hunkListViewRef.loadingModel = false;
        }
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
            bottomIndexTopOffset: 45 // headers and scrollbar space
            items: ListView { model: gitModel.hunks; delegate: Component { Item { } } }
            metricsHelper: Item {
                function get() {
                    hunkLineCache.init(gitModel.hunks.hunkListings, gitModel.hunks.rowCount());
                    var h = 0;
                    var ih;
                    var newHeights = [];
                    var newOffsets = [];
                    var ts = Utils.now();
                    for(var i = 0; i < gitModel.hunks.rowCount(); i++) {
                        var content = LibHelper.modelValue(gitModel.hunks, i, LibHelper.hunks_hunk);
                        var listModel = hunkLineCache.get(i);
                        var txtDims = Style.getTextDims(content, true, true);
                        hunkLineCache.updateLines(i, txtDims.lineHeights);
                        ih = txtDims.height + 20 + 15;
                        newHeights.push(ih);
                        newOffsets.push(h);
                        h += ih;
                    }
//                    console.log("metrics duration", Utils.duration(ts))
                    return {
                        contentHeight: h,
                        heights: newHeights,
                        offsets: newOffsets
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
    HunkLineModelCache {
        id: hunkLineCache
        onLinesModelReady: hunkListViewRef.notify(index)
    }
    ListModel { id: emptyList }
    Component {
        id: hunkTemplate
        Item {
            id: hunkComp
            height: hunkTitleRectRef.height + compTxt.contentHeight + hunkBotScrollRef.height
            width: hunkListViewRef.width
            property int index
            property int linesTo
            property int linesFrom
            property int hunkMaxLineLength
            property int newLineCols
            property int oldLineCols
            property bool atBottom: hunkListViewRef.indexBottom === index
            property ListModel lineHeights
            function load(elmId, idx) {
                if (idx > -1) {
                    var txt = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_hunk);
                    if (!txt) {
                        throw new Error('undefined text for ', idx)
                    }
                    linesFrom = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_linesFrom);
                    linesTo = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_linesTo);
                    hunkMaxLineLength = LibHelper.modelValue(gitModel.hunks, idx, LibHelper.hunks_hunkMaxLineLength);
                    var linesData = hunkLineCache.get(idx);
                    lineHeights = linesData.list;
                    compTxt.text = txt;
                    index = idx;
                    if (linesData.ready) {
                        console.log("notify", index)
                        notify()
                    }
                } else {
                    lineHeights = emptyList;
                    compTxt.text = '';
                }
            }
            function notify() {
                newLineCols = lineHeights.get(0).newLineColumns;
                oldLineCols = lineHeights.get(0).oldLineColumns;
                lineCanvas.requestPaint();
                lineNumCanvas.requestPaint();
            }
            Rectangle {
                id: hunkTitleRectRef
                x: lineNumCanvas.width
                y: 0
                width: parent.width
                height: 20
                color: "transparent"
                TextElement {
                    property bool decodeError: linesTo === 0 && linesFrom === 0
                    function getLineText() {
                        return decodeError ? " : failed to decode hunk as UTF-8"
                                           : " : " + (linesTo - linesFrom + 1) + " lines";
                    }
                    x: 0
                    y: 4
                    opacity: 0.6
                    color: decodeError ? "red" : "black"
                    text: "Hunk " + (index + 1) + getLineText()
                }
            }
            function renderCanvas(canvas, lineNums) {
                var ctx = canvas.getContext('2d');
                var y = 0;
                if (lineNums) {
                    ctx.fillStyle = Qt.rgba(0,0,0,0.5);
                    ctx.textAlign = 'right';
                    ctx.font = '8pt Consolas';
                } else {
                    ctx.font = 'bold 10pt Consolas';
                    ctx.textAlign = 'start';
                }
                ctx.clearRect(0, 0, canvas.width, canvas.height)
                var oldColOffset = oldLineCols * Style.fontFixedWidth + (oldLineCols > 0 ? 2 : 0) * Style.fontFixedWidth;
                var newColOffset = oldColOffset + 2 * Style.fontFixedWidth + newLineCols * Style.fontFixedWidth;
                for(var i = 0; i < lineHeights.count; i++) {
                    var line = lineHeights.get(i);
                    var txtOffset = y + line.height / 2 + 2.5
                    if (!lineNums) {
                        ctx.fillStyle = Style.lineOriginColor(line.origin);
                        ctx.fillRect(0, y, width, line.height);
                        ctx.fillStyle = Qt.rgba(0,0,0,0.5);
                        ctx.fillText(Style.lineOriginSigil(line.origin), 2, txtOffset);
                    } else {
                        if (line.oldLine > 0) {
                            ctx.fillText(line.oldLine, oldColOffset, txtOffset);
                        }
                        if (line.newLine > 0) {
                            ctx.fillText(line.newLine, newColOffset, txtOffset);
                        }
                    }
                    y += line.height;
                }
            }
            LayoutHelper {
                function getWidth() {
                    var add = (2 + (oldLineCols === 0 ? 0 : 2) + (newLineCols === 0 ? 0 : 2)) * Style.fontFixedWidth
                    return oldLineCols * Style.fontFixedWidth + newLineCols * Style.fontFixedWidth + add;
                }
                x: 0
                y: 20
                height: compTxt.contentHeight
                width:  getWidth()
                enabled: false
                Canvas {
                    id: lineNumCanvas
                    height: parent.height
                    width: parent.width
                    onPaint: {
                        if (lineHeights && available) {
                            renderCanvas(lineNumCanvas, true);
                        }
                    }
                }
            }
            Rectangle {
                id: diffRect
                x: lineNumCanvas.width
                y: 20
                width: parent.width - x
                height: compTxt.contentHeight
                clip: true
                Rectangle {
                    id: diffContainerRectRef
                    x: -hunkBotScrollRef.position * width
                    width: compTxt.width
                    height: compTxt.contentHeight
                    color: "transparent"
                    Canvas {
                        id: lineCanvas
                        height: compTxt.contentHeight
                        width: compTxt.width + 10
                        onPaint: {
                            if (lineHeights && available) {
                                renderCanvas(lineCanvas, false);
                            }
                        }
                    }
                    LayoutHelper {
                        x: 0
                        y: 0
                        width: compTxt.width
                        height: compTxt.height
                        enabled: false
                        TextElement {
                            id: compTxt
                            width: Math.max(diffRect.width, contentWidth + 20)
                            height: contentHeight
                            selectableText: true
                            fixedWidthFont: true
                            leftPadding: 10
                            rightPadding: 10
                        }
                    }
                }
            }
            CustomScrollBar {
                id: hunkBotScrollRef
                x: diffRect.x
                function getYOffset() {
                    if (atBottom && enabled) {
                        return hunkListViewRef.height - hunkComp.parent.y - height;
                    }
                    return parent.height - height;
                }
                y: getYOffset()
                policy: ScrollBar.AlwaysOn
                orientation: Qt.Horizontal
                height: 15
                enabled: size < 1
                width: parent.width - x
                size: width / diffContainerRectRef.width
//                stepSize: 1 / (hunkMaxLineLength * 0.5)
//                scrollContainerSize: parent.width
//                scrollContentSize: diffContainerRectRef.width
            }
        }
    }
}
