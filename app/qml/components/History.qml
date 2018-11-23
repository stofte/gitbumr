import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Controls 1.4 as QQC14
import "../base"
import "../style"

Rectangle {
    color: Style.window
    ListModel {
        id: libraryModel
        ListElement {
            title: "A Masterpiece"
            author: "Gabriel"
        }
        ListElement {
            title: "Brilliance"
            author: "Jens"
        }
        ListElement {
            title: "Outstanding"
            author: "Frederik"
        }
    }
    QQC14.TableView {
        anchors.fill: parent
        QQC14.TableViewColumn {
            role: "title"
            title: "Title"
            width: 100
        }
        QQC14.TableViewColumn {
            role: "author"
            title: "Author"
            width: 200
        }
        model: libraryModel
    }
}
