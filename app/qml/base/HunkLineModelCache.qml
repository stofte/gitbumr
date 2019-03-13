import QtQuick 2.11

Item {
    id: root
    signal linesModelReady(int index)
    function init(lineStr, count) {
        internal.initCount(count);
        jsonParser.sendMessage({
            type: 'init',
            value: lineStr,
            models: internal.cache
        });
    }
    function updateLines(index, lhs) {
        jsonParser.sendMessage({
            type: 'lines',
            value: lhs,
            index: index
        });
    }
    function get(index) {
        return {
            list: internal.cache[index],
            ready: internal.ready[index]
        };
    }
    Item {
        id: internal
        function initCount(count) {
            var i;
            for (i = 0; i < ready.length; i++) {
                ready[i] = false;
            }
            var missingCount = count - cache.length;
            for (i = 0; i < missingCount; i++) {
                cache.push(component.createObject(root));
                ready.push(false);
            }
        }
        property variant ready: []
        property variant cache: []
    }
    Component {
        id: component
        ListModel { }
    }
    WorkerScript {
        id: jsonParser
        source: "../scripts/jsonParser.js"
        onMessage: {
            if (messageObject.type === 'ready') {
                var idx = messageObject.value;
                internal.ready[idx] = true;
                root.linesModelReady(idx);
            }
        }
    }
}
