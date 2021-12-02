// goal: 1 liners :)
const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();

// parse input
function lineToClaim(l) {
    const parts = _.split(_.replace(l, ':', ''), ' ');
    const id = _.replace(parts[0], '#', '');
    const xy = _.split(parts[2], ',')
    const wh = _.split(parts[3], 'x')
    
    return {
        id,
        x: _.toNumber(xy[0]),
        y: _.toNumber(xy[1]),
        w: _.toNumber(wh[0]),
        h: _.toNumber(wh[1]),
    }
}

// get bounds
const claims = _.map(lines, lineToClaim);
const rightClaim = _.maxBy(claims, c => c.x + c.w)
const bottomClaim = _.maxBy(claims, c => c.y + c.h)

// build occupancy grid
const occupants = Utils.makeArray2D(rightClaim.x + rightClaim.w, bottomClaim.y + bottomClaim.h, () => new Set());
for (let claim of claims) {
    for (let i = claim.x; i < claim.x + claim.w; i++) {
        for (let j = claim.y; j < claim.y + claim.h; j++) {
            occupants[j][i].add(claim.id);
        }
    }
}

const flat = _.flatten(occupants);

// number of squares with more than one occupant
const part1 = _.size(_.filter(flat, s => s.size > 1));
Utils.part1(part1)

const collisions = {};

for (let s of flat) {
    s.forEach(v => {
        if (!collisions[v]) {
            collisions[v] = new Set();
        }

        s.forEach(w => {
            collisions[v].add(w)
        })  
    })
}

// which square only collides with itself?
const part2 = _.first(_.map(_.filter(_.values(collisions), s => s.size === 1), v => Array.from(v)))
Utils.part2(part2)
