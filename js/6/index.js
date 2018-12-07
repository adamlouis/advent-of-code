const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();

const coords = _.map(lines, l => {
    const p = _.split(l, ', ');
    return {
        x: _.toNumber(p[0]),
        y: _.toNumber(p[1]),
    }
})

const getMins = (l) => {
    let mins = [];
    let minVal = Number.MAX_SAFE_INTEGER;

    for (let v of l) {
        if (minVal > v.value) {
            minVal = v.value;
            mins = [];
        }

        if (minVal === v.value) {
            mins.push(v);
        }
    }
    return mins;
}

const VACANT = '.'
function populateGrid(g, xOffset, yOffset) {
    const h = g.length;
    const w = g[0].length;

    for (let j = 0; j < h; j++) {
        for (let i = 0; i < w; i++) {
            const distances = _.map(coords, (c, idx) => {
                const manhattan = Math.abs(c.x - (i - xOffset)) + Math.abs(c.y - (j - yOffset));
                return {    
                    id: `${idx}`,
                    value: manhattan,
                }
            });

            const mins = getMins(distances);
            const result = {
                minId: mins.length === 1 ? mins[0].id : VACANT,
                sum: _.sumBy(distances, 'value')
            }
            g[j][i] = result;
        }
    }

    return g;
}

function getBorders(g) {
    const borders = new Set([VACANT]);

    const h = g.length;
    const w = g[0].length;

    for (let i = 0; i < w; i++) {
        borders.add(g[0][i].minId)
        borders.add(g[h - 1][i].minId)
    }

    for (let j = 0; j < h; j++) {
        borders.add(g[j][0].minId)
        borders.add(g[j][w - 1].minId)
    }

    return borders;
}

const maxX = _.max(_.map(coords, 'x'));
const maxY = _.max(_.map(coords, 'y'));

const extended = Utils.makeArray2D(maxX * 2, maxY * 2, undefined);
populateGrid(extended, Math.floor(maxX / 2), Math.floor(maxY / 2))

const borders = getBorders(extended);
const counts = _.map(_.countBy(_.map(_.flatten(extended)), 'minId'), (count, id) => ({ count, id }));
const maxArea = _.maxBy(counts, (c) => borders.has(c.id) ? -1 : c.count).count;
Utils.part1(maxArea)

// const THRESHOLD = 32;
const THRESHOLD = 10000;
const part2 = _.size(_.filter(_.map(_.flatten(extended), 'sum'), n => n < THRESHOLD))
Utils.part2(part2)
