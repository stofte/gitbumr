import QtQuick 2.0
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.3
import RustCode 1.0
import "../base"
import "../style"

Pane {
    height: 400
    // move git stuff to a proper parent GitView and with children History/BranchView
    property string gitPath;
    onGitPathChanged: {
        gitModel.load(gitPath);
    }
    Git {
        id: gitModel
    }
    ListView {
        property alias branches: gitModel.branches
        anchors.fill: parent
        Component {
            id: gitDelegate
            Item {
                height: 20
                TextItem {
                    text: name
                    font.bold: checkedout
                }
            }
        }
        model: branches
        delegate: gitDelegate
    }
}
