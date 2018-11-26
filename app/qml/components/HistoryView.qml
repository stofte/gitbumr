import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Controls 1.4 as QQC14
import "../base"
import "../style"

Rectangle {
    color: Style.window
    QQC14.TableView {
        anchors.fill: parent
        QQC14.TableViewColumn {
            role: "oid"
            title: "Oid"
            width: 100
        }
        QQC14.TableViewColumn {
            role: "message"
            title: "Message"
            width: 300
        }
        QQC14.TableViewColumn {
            role: "author"
            title: "Author"
            width: 200
        }
        QQC14.TableViewColumn {
            role: "time"
            title: "Time"
            width: 100
        }
        model: logModel
    }
}
