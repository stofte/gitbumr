import QtQuick 2.9
import QtQuick.Window 2.2
import QtTest 1.0
import GitbumrComponents 1.0

TestCase {
    name: "GitTests"

    Git { id: gitModel }
    Log { id: logModel }
    About { id: proxyTest }

    function test_01_can_load_git_repository() {
        verify(!gitModel.revwalkFilter, "revwalkFilter is empty");
        verify(TST_GIT_PATH, "TST_GIT_PATH is undefined");
        gitModel.load(TST_GIT_PATH);
        verify(gitModel.revwalkFilter.length > 0, "revwalkFilter is unset");
    }

    function test_02_can_load_log_object() {
        logModel.load(TST_GIT_PATH);
        logModel.filter(gitModel.revwalkFilter);
        verify(logModel.rowCount() > 0, "Failed to load log rows");
    }

    function test_03_qml_int_stub_testing() {
        proxyTest.loadGit(TST_GIT_PATH);
        verify(proxyTest.getFilter().length > 0, "proxy revwalkFilter has value");
    }
}
