const fs = require('fs')
const _ = require('lodash');

const day = _.last(_.split(process.argv[1], '/'))

const Utils = {
    readString: () => fs.readFileSync(`${process.argv[1]}/data.txt`),
    readLines: () => _.split(Utils.readString(), '\n'),
    part1: (v) => console.log(`day=${day} part_1=${v}`),
    part2: (v) => console.log(`day=${day} part_2=${v}`),

    makeArray1D: (n, v) => {
        const a = new Array(n);
        return _.map(a, i => _.isFunction(v) ? v() : v);
    },
    makeArray2D: (w, h, v) => {
        const a = new Array(h);
        return _.map(a, i => Utils.makeArray1D(w, v));
    },

}

module.exports = Utils;