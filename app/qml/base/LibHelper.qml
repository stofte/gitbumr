pragma Singleton
import QtQuick 2.11

Item {
    // note the user role + offset, which is defined in Bindings.cpp.
    // its the json properties sorted alphabetically (and not as defined
    // in binding.json!). This is tricky because even just renaming 
    // properties shuffles their indexes.
    // These values should be used by the model.data(modelIdx, roleValue)
    // method, where role value is the appropiate column value from below.
    readonly property int diffs_filenameNew:        0x100

    readonly property int hunks_hunk:               0x100
    readonly property int hunks_hunkMaxLineLength:  0x101
    readonly property int hunks_lines:              0x102
    readonly property int hunks_linesFrom:          0x103
    readonly property int hunks_linesNewCols:       0x105
    readonly property int hunks_linesOldCols:       0x107
    readonly property int hunks_linesOrigin:        0x108
    readonly property int hunks_linesTo:            0x109

    readonly property int commits_author:           0x100
    readonly property int commits_cid:              0x101
    readonly property int commits_graph:            0x102
    readonly property int commits_isMerge:          0x103
    readonly property int commits_message:          0x104
    readonly property int commits_summary:          0x105
    readonly property int commits_time:             0x106
    readonly property int commits_timeHumanized:    0x107

    function getHunk(model, index) {
        var i = model.index(index, 0);
        var data = {
            hunk: model.data(i, hunks_hunk),
            linesFrom: model.data(i, hunks_linesFrom),
            linesTo: model.data(i, hunks_linesTo),
            linesOldCols: model.data(i, hunks_linesOldCols),
            linesNewCols: model.data(i, hunks_linesNewCols)
        };
        return data;
    }
    function getCommit(model, index) {
        var i = model.index(index, 0);
        var data = {
            author: model.data(i, commits_author),
            cid: model.data(i, commits_cid),
            graph: model.data(i, commits_graph),
            isMerge: model.data(i, commits_isMerge),
            message: model.data(i, commits_message),
            summary: model.data(i, commits_summary),
            time: model.data(i, commits_time),
            timeHumanized: model.data(i, commits_timeHumanized)
        };
        return data;
    }

    function modelValue(model, idx, role) {
        return model.data(model.index(idx, 0), role);
    }
}
