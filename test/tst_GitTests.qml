import QtQuick 2.9
import QtQuick.Window 2.2
import QtTest 1.0
import RustCode 1.0

TestCase {
    name: "GitTests"

    Git { id: gitModel }

    function test_can_load() {
        verify(!gitModel.revwalkFilter, "revwalkFilter is empty");
        verify(TST_GIT_PATH, "TST_GIT_PATH is defined");
        gitModel.load(TST_GIT_PATH);
        verify(gitModel.revwalkFilter.length > 0, "revwalkFilter is set");
    }
}
