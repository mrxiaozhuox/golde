/**
 * Golde
 * YuKun Liu <mrxzx.info@gmail.com>
 * https://github.com/mrxiaozhuox/golde/
 */

setInterval(function() {

    var queue = document.getElementById("GoldeEventQueue").value;
    var queue = JSON.parse(queue);

    var new_queue = {};
    var need_submit = false;

    for (const key in queue) {
        var data = queue[key];
        if (data.result == "None") {
            need_submit = true;
            var result = eval(data.code);
            new_queue[key] = {
                code: data.code,
                result: dataValueParser(result),
            };
        }
    }

    if (need_submit) {
        // console.log("new submit: " + JSON.stringify(new_queue));
        document.getElementById("GoldeEventQueue").value = JSON.stringify(new_queue);
        document.getElementById("GoldeEventQueueSubmit").click();
    }


}, 100);

document.getElementById("GoldeEventQueue").onsubmit = function() {
    return false;
}

function dataValueParser(value) {
    if (typeof value == "boolean") {
        return { "Boolesn": value };
    }
    if (typeof value == "number") {
        return { "Number": value };
    }
    if (typeof value == "string") {
        return { "String": value };
    }
    if (typeof value == "undefined") {
        return { "Number": 0 };
    }
    if (typeof value == "object") {
        if (Array.isArray(value)) {

            var temp = [];
            for (const key in value) {
                temp.push(dataValueParser(value[key]));
            }
            return { "Array": temp };

        } else {

            var temp = {};
            for (const key in value) {
                temp[key] = dataValueParser(value[key]);
            }
            return { "Dict": temp };

        }
    }
}