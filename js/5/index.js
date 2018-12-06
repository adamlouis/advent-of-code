const _ = require('lodash');
const Utils = require('../utils');

const buildLinkedList = (s) => {
    const chars = _.split(s, '');
    const root = {};

    root.next = {
        value: chars[0],
        previous: root,
        next: undefined,
    }

    let previous = root.next;
    for (let i = 1; i < chars.length; i++) {
        previous.next = {
            value: chars[i],
            previous,
            next: undefined,
        }

        previous = previous.next;
    }

    return root;
}

const getLength = (node) => {
    let count = 0;
    let c = node;
    while (c) {
        // assume: root is only node with node value. don't count it.
        if (c.value) {
            count++;
        }

        c = c.next;
    }
    return count;
}

const shouldDestroy = (node) => {
    const c1 = _.get(node, 'value');
    const c2 = _.get(node, 'next.value');

    return (c1 !== c2) && (_.toLower(c1) == c2 || c1 == _.toLower(c2));
};

const react = (init) => {
    let current = init;

    while (current) {
        if (shouldDestroy(current)) {

            const n0 = current.previous;
            const n3 = _.get(current, 'next.next');

            // assume: root is only node with node value. will never be destroyed and n0 always is defined.
            n0.next = n3;
            _.set(n3, 'previous', n0)

            current = n0;
        } else {
            current = current.next;
        }
    }

    return init;
}

const content = Utils.readString();
Utils.part1(getLength(react(buildLinkedList(content))));

const replacements = ['Aa', 'Bb', 'Cc', 'Dd', 'Ee', 'Ff', 'Gg', 'Hh', 'Ii', 'Jj', 'Kk', 'Ll', 'Mm', 'Nn', 'Oo', 'Pp', 'Qq', 'Rr', 'Ss', 'Tt', 'Uu', 'Vv', 'Ww', 'Xx', 'Yy', 'Zz',]
const candidates = _.map(replacements, (r) => _.replace(content, new RegExp(`[${r}]`, 'g'), ''));
const roots = _.map(candidates, s => buildLinkedList(s));
const results = _.map(roots, react);
const lengths = _.map(results, getLength);

Utils.part2(_.min(lengths))









