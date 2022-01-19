/**
 * Golde
 * YuKun Liu <mrxzx.info@gmail.com>
 * https://github.com/mrxiaozhuox/golde/
 */

setInterval(function() {

    if (platform == "WASM") {
        var queue = document.getElementById("GoldeEventQueue").getAttribute("value");
    } else {
        var queue = document.getElementById("GoldeEventQueue").value;
    }

    try {
        var queue = JSON.parse(queue);
    } catch (error) {
        var queue = {};
    }

    var new_queue = {};
    var need_submit = false;

    for (const key in queue) {
        var data = queue[key];
        if (data.result == "None") {
            need_submit = true;
            try {
                var result = eval(data.code);
                new_queue[key] = {
                    code: data.code,
                    result: dataValueParser(result),
                };
            } catch {
                delete new_queue[key];
            }
        }
    }

    if (need_submit) {
        if (platform == "WASM") {
            document.getElementById("GoldeEventQueue").setAttribute("value", JSON.stringify(new_queue));
        } else {
            document.getElementById("GoldeEventQueue").value = JSON.stringify(new_queue);
        }

        document.getElementById("GoldeEventQueueSubmit").click();
    }


}, 50);

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
    if (typeof value == "object") {

        if (Array.isArray(value)) {

            var temp = [];
            for (const key in value) {
                temp.push(dataValueParser(value[key]));
            }
            return { "List": temp };

        } else {

            if (value === null) {
                return { "String": "<Null>" };
            }

            var temp = {};
            Object.keys(value).map(key => {
                temp[key] = dataValueParser(value[key]);
            })
            return { "Dict": temp };

        }
    }

    if (typeof value == "undefined") {
        return { "String": "<Undefined>" };
    }
    if (typeof value == "function") {
        return { "String": "<Function>" };
    }
    if (typeof value == "symbol") {
        return { "String": "<Symbol>" };
    }
    return { "String": "<Unknown>" };
}

function WebAssemblyGetResult() {
    return document.getElementById("GoldeEventQueue").getAttribute("value");
}
