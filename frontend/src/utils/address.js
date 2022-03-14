export function trunc(str, len) {
    const min = 8+3+3;
    if (str.length <= len || len < min) {
        return str;
    }
    return str.substring(0, 8) + "..." + str.substring(str.length-(len-8-3));
}