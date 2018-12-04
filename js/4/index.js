// goal: 1 liners :)
const _ = require('lodash');
const Utils = require('../utils');

const lines = Utils.readLines();

// build event timeline
function linesToEvents(l) {
    const parts = _.split(_.replace(l, '[', ''), '] ');

    const ts = parts[0];
    const text = parts[1];

    return {
        ts,
        text
    }
}
const events = _.sortBy(_.map(lines, linesToEvents), 'ts');

// get stats by guard
const FALL_ASLEEP = 'falls asleep';
const WAKES_UP = 'wakes up';
const GUARD = 'Guard ';

const guards = {};
const getGuardId = (t) => _.toNumber(_.replace(_.split(t, ' ')[1], '#', ''))

let id;
let startTs;

for (let e of events) {
    if (_.startsWith(e.text, GUARD)) {
        id = getGuardId(e.text);
        if (!guards[[id]]) {
            guards[id] = {
                id,
                sleeps: []
            }
        }
    } else if (_.startsWith(e.text, FALL_ASLEEP)) {
        startTs = e.ts;

    } else if (_.startsWith(e.text, WAKES_UP)) {
        const minStart = _.toNumber(_.split(_.split(startTs, ' ')[1], ':')[1]);
        const minEnd = _.toNumber(_.split(_.split(e.ts, ' ')[1], ':')[1]);

        // we only care about minutes in the given input data
        guards[id].sleeps.push({
            start: minStart,
            end: minEnd,
        })
    } else {
        console.log('error parsing!')
        process.exit(1)
    }
}

function getSleepTime(s) {
    return s.end - s.start;
}

// guard who slept the most
const maxSleep = _.maxBy(_.values(guards), g => _.sumBy(g.sleeps, getSleepTime))
const maxGuard = maxSleep.id;

// minute the guard was most often asleep
const totals = Utils.makeArray1D(60, (i) => ({ i, count: 0 }));
for (let s of maxSleep.sleeps) {
    for (let i = s.start; i < s.end; i++) {
        totals[i].count++
    }
}
const maxMinute = _.maxBy(totals, 'count').i;
Utils.part1(maxGuard * maxMinute)

const freqs = Utils.makeArray1D(60, (i) => ({ i, freqs: {} }));
for (let g of _.values(guards)) {
    for (let s of g.sleeps) {
        for (let i = s.start; i < s.end; i++) {
            if (!freqs[i].freqs[g.id]) {
                freqs[i].freqs[g.id] = 0
            }

            freqs[i].freqs[g.id]++
        }
    }
}

const minuteWithMax = _.maxBy(freqs, w => _.max(_.values(w.freqs)))
const maxMinuteSlept = minuteWithMax.i;
const maxGuardAtMinute = _.maxBy(_.keys(minuteWithMax.freqs), k => minuteWithMax.freqs[k])

Utils.part2(maxMinuteSlept*maxGuardAtMinute)
