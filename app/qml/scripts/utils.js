function duration(start) {
    return now() - start;
}

function now() {
    return new Date().getTime()
}

function hypot(a, b) {
    return Math.sqrt(a * a + b * b);
}
