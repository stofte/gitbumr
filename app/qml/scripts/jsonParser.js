var modelList = [];

WorkerScript.onMessage = function(message) {
    var i, j;
    if (message.type === 'init') {
        modelList = message.models;
        // reset models
        for (i = 0; i < modelList.length; i++) {
            modelList[i].clear();
        }
        var json = JSON.parse(message.value);
        if (modelList.length < json.length) {
            throw new Error('not enough models:' + modelList.length + ' needed:' + json.length);
        }
        for (i = 0; i < json.length; i++) {
            var l = json[i];
            for (j = 0; j < l.length; j+=3) {
                var row = {
                    oldLine: l[j],
                    newLine: l[j+1],
                    origin: l[j+2],
                    height: 10
                };
                modelList[i].append(row);
            }
            modelList[i].sync();
        }
    } else if (message.type === 'lines') {
        var idx = message.index;
        var lineList = modelList[idx];
        if (message.value.length !== lineList.count) {
            throw new Error('mismatched model and line length', lineList.length, 'vs', message.value);
        }
        for (i = 0; i < lineList.count; i++) {
            lineList.setProperty(i, 'height', message.value[i]);
        }
        lineList.sync();
        WorkerScript.sendMessage({ type: 'ready', value: idx });
    }
}
