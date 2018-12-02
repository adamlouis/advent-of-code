const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();

let two = 0;
let three = 0;

function hasN(s, n) {
    const freqs = {}
    for (let c of s) {
        if (!freqs[c]) {
            freqs[c] = 1;
        } else {
            freqs[c]++;
        }
    }

    return _.some(_.values(freqs), v => v === n)
}

for (let l of lines) {
    if (hasN(l, 2)) {
        two++;
    }
    if (hasN(l, 3)) {
        three++;
    }
}

console.log(`part 1:${two*three}`)

function distance(s1, s2) {
    let count = 0;
    const n = _.max([s1.length, s2.length]);
    for (let i = 0; i < n; i++) {
        if (_.get(s1, i) !== _.get(s2, i)) {
            count++;
        }
    }
    return count;
}

function common(s1, s2) {
    let result = '';
    const n = _.max([s1.length, s2.length]);
    for (let i = 0; i < n; i++) {
        if (_.get(s1, i) == _.get(s2, i)) {
            result += _.get(s1, i);
        }
    }
    return result;
}

for (var i = 0; i < lines.length; i++) {
    for (var j = i + 1; j < lines.length; j++) {
        if (distance(lines[i], lines[j]) == 1) {
            console.log(`part 2: ${common(lines[i], lines[j])}`)
        }
    }
}
