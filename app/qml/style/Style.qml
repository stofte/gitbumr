pragma Singleton
import QtQuick 2.9
import "../base"

Item {
    id: root
    function gitStatusColor(status) {
        switch (status) {
            case "Modified":    return "#F6C342";
            case "Added":       return "#14892C";
            case "Deleted":     return "#D04437";
            case "Renamed":     return "#AC707A";
            case "":            return root.window;
            default:
                throw new Error('gitStatusColor unhandled git status: ' + status);
        }
    }
    // see lib/src/implementation/hunk.rs for encoding of origin vals
    function lineOriginSigil(val) {
        switch (val) {
            case 0: return ' ';
            case 1: return '+';
            case 2: return '-';
            case 3: return '<';
            case 4: return '>';
            case 5: return '=';
            default:
                throw new Error("unhandled case: '" + val + "' in lineOriginSigil");
        }
    }
    function lineOriginColor(val) {
        switch (val) {
            case 1: return '#DDFFDD';
            case 2: return '#FEE8E9';
            case 0:
            case 3:
            case 4:
            case 5:
                return '#FFFFFF';
            default:
                throw new Error("unhandled case: '" + val + "' in lineOriginColor");
        }
    }
    function getTextDims(txt, useFixedWidthFont, lineModel) {
        var txtElm = useFixedWidthFont ? txtFixedSizeRef : txtSizeRef;
        txtElm.text = txt;
        var w = txtElm.contentWidth;
        var h = txtElm.contentHeight;
        var lh = txtElm.contentHeight / txtElm.lineCount;
        var y = 2;
        var pos = -1;
        var lineHeights = [];
        while (lineModel && true) {
            var newPos = txtElm.positionAt(10000, y);
            var rect = txtElm.positionToRectangle(newPos)
            if (newPos === pos) {
                break;
            }
            y += rect.height;
            lineModel.append({value: rect.height});
            pos = newPos;
        }
        return {
            width: w,
            height: h,
            lineHeight: lh,
        };
    }
    FontLoader { id: mainFont; name: "Roboto" }
    FontLoader { id: fixedWidthFont; name: "Consolas" }
    SystemPalette { id: palette; colorGroup: SystemPalette.Active }

    readonly property int fontPointSize: 10
    readonly property int fontFixedPointSize: 8
    readonly property int borderSize: 4

    readonly property string dark: palette.dark
    readonly property string mid: palette.mid
    readonly property string light: palette.light
    readonly property string window: "#FAFAFA"
    readonly property string control: "#D3D3D3"
    readonly property string controlActive: "#A8A8A8"
    readonly property string selection: "#0078D7"

    readonly property int fontRendering: Text.QtRendering
    readonly property string fontName: mainFont.name
    readonly property string fontNameFixedWidth: fixedWidthFont.name

    readonly property real tabStopSize: onceOnlyRef.onceTabStopSize
    readonly property real tabStopFixedSize: onceOnlyRef.onceTabStopFixedSize
    readonly property real fontFixedLineHeight: onceOnlyRef.oncFontFixedLineHeight

    Item {
        // This avoids setting unneeded dynamic bindings that qt  will complain about eventually,
        // but we also maintain readonly attributes in the main Style interface.
        id: onceOnlyRef
        property real onceTabStopSize: 0
        property real onceTabStopFixedSize: 0
        property real oncFontFixedLineHeight: 0
        Component.onCompleted: {
            onceTabStopSize = getTextDims("\t", false).width;
            onceTabStopFixedSize = getTextDims("1234", true).width;
            oncFontFixedLineHeight = getTextDims("1234", true).lineHeight;
        }
    }
    TextEdit {
        id: txtSizeRef
        renderType: fontRendering
        font.family: fontName
        font.pointSize: fontPointSize
        textFormat: TextEdit.PlainText
        text: ""
    }
    TextEdit {
        id: txtFixedSizeRef
        renderType: fontRendering
        font.family: fontNameFixedWidth
        font.pointSize: fontFixedPointSize
        textFormat: TextEdit.PlainText
        text: "1234"
    }
}
