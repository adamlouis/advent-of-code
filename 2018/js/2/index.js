// goal: 1 liners :)
const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();

// part 1
const hasN = (s, n) => _.some(_.values(_.countBy(s)), v => v === n);

const two = _.sumBy(lines, l => hasN(l, 2));
const three = _.sumBy(lines, l => hasN(l, 3));
Utils.part1(two*three)

// part 2
function distance(s1, s2) {
    const zipped = _.zip(s1.split(''), s2.split(''))
    return _.sumBy(zipped, (z) => _.get(z, 0) !== _.get(z, 1));
}

function common(s1, s2) {
    const zipped = _.zip(s1.split(''), s2.split(''))
    const matches = _.filter(zipped, z => _.get(z, 0) === _.get(z, 1))
    return _.join(_.map(matches, m => _.get(m, 0)), '');
}

const part2 = _.first(_.compact(_.flatten(_.map(lines, (l1, i) => _.map(lines.slice(i), l2 => distance(l1, l2) === 1 ? common(l1, l2) : '')))));
Utils.part2(part2)
