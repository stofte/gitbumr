import QtQuick 2.11
import "../style"

TextEdit {
    property bool selectableText: false
    // seems that if text elements overlap, renderType switches to QtRendering?
    // eg, seen in hunkview linenumber listings
    renderType: Text.QtRendering
    font.pointSize: Style.fontPointSize
    font.family: Style.fontName
    readOnly: true
    selectByMouse: selectableText
}
