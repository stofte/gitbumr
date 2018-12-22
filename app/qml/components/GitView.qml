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
        // when we update the git model, the revwalk filter will update, firing
        // onRevwalkFilterChanged. to avoid being called before the git ref is set
        // in logModel, we call load on the logModel as the first thing.
        logModel.load(gitPath);
        gitModel.load(gitPath);
    }

    Git {
        id: gitModel
        onRevwalkFilterChanged: {
            logModel.filter(revwalkFilter);
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
        Layout.preferredWidth: parent.width * 0.5
    }

    QQC14.SplitView {
        orientation: Qt.Horizontal
        handleDelegate: Rectangle {
            color: Style.dark
            width: 1
        }
        HistoryView {
            id: historyViewRef
            width: parent.width * 0.5
            clip: true
            onSelectedChanged: {
                commitRef.commitId = selected;
            }
        }
        Commit {
            id: commitRef
            width: parent.width * 0.5
        }
    }
}
