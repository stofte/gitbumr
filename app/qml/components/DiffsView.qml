import QtQuick 2.0
import RustCode 1.0
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Item {
    ListView {
        clip: true
        anchors.fill: parent
        model: gitModel.diffs
        interactive: false
        delegate: Component {
            ColumnLayout {
                TextEdit {
                    id: textRef
                    font.family: Style.fontNameFixedWidth
                    width: parent.width
                    wrapMode: Text.NoWrap
                    readOnly: true
                    selectByMouse: true
                    text: filename
                }
                TextEdit {
                    id: diffRef
                    width: parent.width
                    font.family: Style.fontNameFixedWidth
                    textFormat: Text.RichText
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
}
