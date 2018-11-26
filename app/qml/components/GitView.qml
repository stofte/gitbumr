import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Controls 1.4 as QQC14
import QtQuick.Layouts 1.3
import RustCode 1.0
import "../style"
import "../base"

QQC14.SplitView {
    height: 400
    property string gitPath;
    onGitPathChanged: {
        console.log("onGitPathChanged", gitPath)
        // when we update the git mode, the revwalk filter will update, firing
        // onRevwalkFilterChanged. to avoid being called before the git ref is set,
        // we call load on the logModel as the first thing.
        logModel.load(gitPath);
        gitModel.load(gitPath);
    }

    Git {
        id: gitModel
        onRevwalkFilterChanged: {
            console.log("onRevwalkFilterChanged", revwalkFilter);
            if (revwalkFilter) {
                logModel.filter(revwalkFilter);
            }
        }
    }

    Log {
        id: logModel
    }

    orientation: Qt.Horizontal
    anchors.fill: parent
    handleDelegate: Rectangle {
        color: Style.dark
        width: 1
    }
    BranchView {
        id: branchView
        Layout.fillHeight: true
        Layout.minimumWidth: 100
        Layout.preferredWidth: 200
    }
    HistoryView {
        Layout.fillHeight: true
    }
}
