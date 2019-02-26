import QtQuick 2.9
import QtQuick.Window 2.2
import QtTest 1.0
import RustCode 1.0

TestCase {
    name: "GitTests"

    Git { id: gitModel }

    function test_can_load() {
        gitModel.load("C:/src/gitbumr");
    }
}
