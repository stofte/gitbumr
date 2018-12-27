import QtQuick 2.0
import RustCode 1.0
import "../base"
import "../style"

Item {
    clip: true
    Column {
        TextItem {
            text: "Commit: " + gitModel.commit.cid
        }
        TextItem {
            text: "Author: " + gitModel.commit.author
        }
        TextItem {
            text: "Committer: " + gitModel.commit.comitter
        }
        TextItem {
            text: "Time: " + gitModel.commit.time
        }
        TextItem { text: " " }
        TextItem {
            text: gitModel.commit.message
        }
    }
}
