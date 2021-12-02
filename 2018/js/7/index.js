const _ = require('lodash');
const Utils = require('../utils');
const lines = Utils.readLines();

const nodes = {}

for (let line of lines) {
    const fields = _.split(line, ' ');
    const parent = fields[1];
    const child = fields[7];

    if (!nodes[parent]) {
        nodes[parent] = { id: parent, children: [], parents: [] }
    }

    if (!nodes[child]) {
        nodes[child] = { id: child, children: [], parents: [] }
    }

    nodes[child].parents.push(parent)
    nodes[parent].children.push(child)
}

_.forEach(nodes, n => {
    n.parents = _.sortBy(n.parents);
    n.children = _.sortBy(n.children);
});

function getOrder(nodeValues) {
    const result = [];
    const seen = new Set();
    
    while (result.length < _.size(nodeValues)) {
        const available = _.filter(nodeValues, v => _.every(v.parents, p => seen.has(p)) && !seen.has(v.id));
        result.push(available[0].id);
        seen.add(available[0].id);
    }
   
    return result;
}

const nodeValues = _.sortBy(_.values(nodes), 'id');
const order = getOrder(nodeValues);
Utils.part1(_.join(order, ''))

// data.1.txt
// const getDuration = (s) => s.charCodeAt(0) - 64;
// const WORKER_COUNT = 2;

// data.2.txt
const getDuration = (s) => s.charCodeAt(0) - 4;
const WORKER_COUNT = 5;

const tasks = _.map(order, n => ({
    id: n,
    started: false,
    remaining: getDuration(n),
    parents: nodes[n].parents,
}))

let time = 0;
let doneSet = new Set();
let doing = []

while (doneSet.size < _.size(nodes)) {
    const available = _.filter(tasks, v => _.every(v.parents, p => doneSet.has(p)) && !v.started);
    const todo = _.slice(available, 0, WORKER_COUNT - _.size(doing));

    _.forEach(todo, t => t.started = true);
    doing = _.concat(doing, todo);

    _.forEach(doing, d => {
        d.remaining--;
        if (d.remaining == 0) {
            doneSet.add(d.id);
        }
    })
    doing = _.filter(doing, d => d.remaining > 0)

    time++;
}

Utils.part2(time)
