const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();
const numbers = _.map(lines, s => _.toFinite(_.replace(s, '+', '')));

const part1 = _.sum(numbers)
Utils.part1(part1)

const seen = new Set();
let f = 0;
let i = 0;

while (true) {
    const v = numbers[i];
    f += v;

    if (seen.has(f)) {
        Utils.part2(f)
        process.exit(1)
    }

    seen.add(f)
    i = (i + 1) % numbers.length;
}
