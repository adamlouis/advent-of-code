const fs = require('fs')
const _ = require('lodash');

const Utils = {
    readLines: (path) => _.split(fs.readFileSync(path), '\n'),
}


module.exports = Utils;