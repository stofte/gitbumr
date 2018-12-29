import QtQuick 2.0
import "../base"
import "../style"

ListView {
    property ListModel hunkModel: hunkListRef
    ListModel {
        id: hunkListRef
        ListElement { line: 1 }
        ListElement { line: 2 }
        ListElement { line: 3 }
    }
    model: hunkModel
    delegate: Component {
        Item {
            height: diffRef.contentHeight + 10
            TextEdit {
                id: diffRef
                width: parent.width
                font.family: Style.fontNameFixedWidth
                //textFormat: Text.StyledText
                readOnly: true
                selectByMouse: true
                text: "<table>
  <tr>
    <td bgcolor='red'>Jill</td>
    <td>Smith</td>
    <td>50</td>
  </tr>
  <tr>
    <td>Eve</td>
    <td>Jackson</td>
    <td>94</td>
  </tr>
</table>"
            }
        }
    }
}
