const fs = require('fs')
const _ = require('lodash');

const Utils = {
    readString: () => fs.readFileSync(`${process.argv[1]}/data.txt`),
    readLines: () => _.split(Utils.readString(), '\n'),
}

module.exports = Utils;