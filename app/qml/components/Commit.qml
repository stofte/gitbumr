import QtQuick 2.0
import RustCode 1.0

Item {
    Column {
        Text {
            text: "Commit: " + gitModel.commit.cid
        }
        Text {
            text: "Author: " + gitModel.commit.author
        }
        Text {
            text: "Committer: " + gitModel.commit.comitter
        }
        Text {
            text: "Time: " + gitModel.commit.time
        }
        Text { text: " " }
        Text {
            text: gitModel.commit.message
        }
    }
}
