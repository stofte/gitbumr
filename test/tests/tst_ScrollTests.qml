import QtQuick 2.11
import QtTest 1.0
import GitbumrComponents 1.0

TestCase {
    name: "ScrollTests"
    Item {
        id: scrollcontainer
        height: 100
        width: 100
        Rectangle {
            id: scrolltarget
            height: 200
            width: 100
        }
    }
    ScrollBar2 {
        id: scrollbar
        target: scrolltarget
        container: scrollcontainer
    }

    function test_01_default_orientation_is_vertical() {
        verify(scrollbar.orientation === Qt.Vertical, "Default orientation is vertical");
    }
}
