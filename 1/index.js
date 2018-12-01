const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines('1/data.txt');
const numbers = _.map(lines, s => _.toFinite(_.replace(s, '+', '')));

const part1 = _.sum(numbers)
console.log('part 1:', part1)

const seen = new Set();
let f = 0;
let i = 0;

while (true) {
    const v = numbers[i];
    f += v;

    if (seen.has(f)) {
        console.log('part 2:', f)
        process.exit(1)
    }

    seen.add(f)
    i = (i + 1) % numbers.length;
}
