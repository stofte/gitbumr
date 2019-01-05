import QtQuick 2.11
import "../style"

TextEdit {
    property bool selectableText: false
    property bool fixedWidthFont: false
    property bool isTextElement: selectableText
    // seems that if text elements overlap, renderType switches to QtRendering?
    // eg, seen in hunkview linenumber listings
    renderType: Text.QtRendering
    font.pointSize: fixedWidthFont ? Style.fontFixedPointSize : Style.fontPointSize
    font.family: fixedWidthFont ? Style.fontNameFixedWidth : Style.fontName
    readOnly: true
    selectByMouse: selectableText
    tabStopDistance: fixedWidthFont ? Style.tabStopFixedSize : Style.tabStopSize
    onTextChanged: {
        //console.log(Style.getTextWidth(text, false));
    }
}
