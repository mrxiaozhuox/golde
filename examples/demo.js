function value_type(type) {
    if (type == "string") {
        return "hello world";
    } else if (type == "number") {
        return 1;
    } else if (type == "boolean") {
        return true;
    } else if (type == "dict") {
        return {
            name: "YuKun Liu",
            age: 18
        };
    } else if (type == "list") {
        return [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    }
}