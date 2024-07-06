#!/usr/bin/env python3

# folders:
#   Nouns & Pronouns:      ...
#   Actions:    words that convey action -- bring, walk, run, learn.
#   States:     words that convey state -- be, exist, stand.
#   Attributes: words that describe or define another word.
#   Modifiers:  words that modify another word -- e.g. by specifying manner, place, time, etc.
#   
#   Interjections: 
#   Conjunction: ... (persistent)

from sortedcontainers import SortedSet

import csv

with open("words.tsv") as tsvfile:
    reader = csv.reader(tsvfile, delimiter="\t")
    headers = next(reader)

    words = dict(map(lambda x: [x, SortedSet()], headers[1:]))
    for row in reader:
        word = row[0]
        for idx in range(1, len(headers)):
            group = headers[idx]
            in_group = bool(row[idx])
            if in_group:
                words[group].add(word)

def cats(*folders):
    global words # i know, i know, i'm sorry
    results = SortedSet()
    for category in folders:
        results.update(words[category])
    return results

folders = {
    'Nouns/Actions': cats('noun.act'),
    'Nouns/Artifacts': cats('noun.artifact'),
    'Nouns/Attributes': cats('noun.attribute'),
    'Nouns/Body': cats('noun.body'),
    'Nouns/Cognition': cats('noun.cognition'),
    'Nouns/Communication': cats('noun.communication').difference({'a', 'u'}),
    'Nouns/Events': cats('noun.event'),
    'Nouns/Feelings': cats('noun.feeling'),
    'Nouns/Food': cats('noun.food'),
    'Nouns/Group': cats('noun.group'),
    'Nouns/Location': cats('noun.location'),
    'Nouns/Motives': cats('noun.motive'),
    'Nouns/Object': cats('noun.object'),
    'Nouns/People': cats('noun.person'),
    'Nouns/Plants': cats('noun.plant'),
    'Nouns/Possession': cats('noun.possession'),

    'Attributes': cats('adj.all'),
    'Modifiers': cats('adv.all'),
}

folders['Pronouns'] = SortedSet([
    'I',
    'you',
    'it',
    'she',
    'he',
    'they',
    'one',
    'own',
    'self',
    'this',
    'that',
    'we',
])

folders['Nouns/Animals/Body Parts'] = SortedSet([
    'antenna',
    'breast',
    'coat',
    'down',
    'fin',
    'flank',
    'foot',
    'fur',
    'hair',
    'head',
    'hide',
    'horn',
    #'knot', # It technically belongs here and WordNet has it under noun.animal.
    'lip',
    'pad',
    'scale',
    'shell',
    'sole',
    'tail',
    'throat',
    'tooth',
    'valve',
    'vein',
    'wing',
    'wool',
])

folders['Nouns/Animals'] = cats('noun.animal').difference(folders['Nouns/Animals/Body Parts'], {
    'assault',
    'bay',
    'beef',
    'blue',
    'cancer',
    'cannon',
    'carrier',
    'citation',
    'coat',
    'copper',
    'dam',
    'down',
    'drum',
    'emperor',
    'flag',
    'game',
    'giant',
    'grade',
    'grey',
    'host',
    'hybrid',
    'investment',
    'jersey',
    'layer',
    'migrator',
    'monarch',
    'monitor',
    'monster',
    'pen',
    'permit',
    'pointer',
    'poll',
    'predator',
    'prey',
    'primary',
    'rail',
    'relative',
    'rod',
    'royal',
    'style',
    'soldier',
    'survivor',
    'test',
    'vector',
    'web',
    'worker',
    'world',
})

folders['Nouns/Remaining'] = cats(
        'noun.act', 'noun.animal', 'noun.artifact', 'noun.attribute',
        'noun.body', 'noun.cognition', 'noun.communication', 'noun.event',
        'noun.feeling', 'noun.food', 'noun.group', 'noun.location',
        'noun.motive', 'noun.object', 'noun.person', 'noun.phenomenon',
        'noun.plant', 'noun.possession', 'noun.process', 'noun.quantity',
        'noun.relation', 'noun.shape', 'noun.state', 'noun.substance',
        'noun.time').difference(
    folders['Nouns/Actions'],
    folders['Nouns/Artifacts'],
    folders['Nouns/Attributes'],
    folders['Nouns/Body'],
    folders['Nouns/Cognition'],
    folders['Nouns/Communication'],
    folders['Nouns/Events'],
    folders['Nouns/Feelings'],
    folders['Nouns/Food'],
    folders['Nouns/Group'],
    folders['Nouns/Location'],
    folders['Nouns/Motives'],
    folders['Nouns/Object'],
    folders['Nouns/People'],
    folders['Nouns/Plants'],
    folders['Nouns/Possession'],
    folders['Nouns/Animals/Body Parts'],
    folders['Nouns/Animals'],
)

#    'body': cats('verb.body'),
#    'change': cats('verb.change'),
#    'cognition': cats('verb.cognition'),
#    'communication': cats('verb.communication'),
#    'competition': cats('verb.competition'),
#    'consumption': cats('verb.consumption'),
#    'contact': cats('verb.contact'),
#    'creation': cats('verb.creation'),
#    'emotion': cats('verb.emotion'),
#    'motion': cats('verb.motion'),
#    'perception': cats('verb.perception'),
#    'possession': cats('verb.possession'),
#    'social': cats('verb.social'),
#    'stative': cats('verb.stative'),
#    'weather': cats('verb.weather'),

from pprint import pprint

def set_button_parent(parent, btn):
    if btn is None:
        return btn

    btn['parent'] = parent
    return btn

def folder(name, buttons, toplevel=False, immediate=False, rows=6, cols=9):
    return {
        "name": name.split("::")[-1],
        "id": name,
        "toplevel": toplevel,
        "immediate": immediate,
        "rows": rows,
        "cols": cols,
        "buttons": [set_button_parent(name, btn) for btn in buttons],
    }

def button(word, **options):
    if 'folder' in options and options['folder']:
        options['folder'] = word
        word = word.split('::')[-1]
    return {'label': word, **options}

def buttons(words, **options):
    remaining = []
    if options.get('folder', False):
        remaining = [None] * ((6 * 9) - len(words))
    return [button(word, **options) for word in words] + remaining

def folder_button(word, **options):
    return button(word, folder=True, **options)

nouns_subfolders = [
    folder('Nouns::Animals',   buttons(['Nouns::Animals::Body Parts'], folder=True) + buttons(folders['Nouns/Animals'])),
    folder('Nouns::Body',      buttons(folders['Nouns/Body'])),
    folder('Nouns::Actions',   buttons(folders['Nouns/Actions'])),
    folder('Nouns::Artifacts', buttons(folders['Nouns/Artifacts'])),
    folder('Nouns::Cognition', buttons(folders['Nouns/Cognition'])),
    folder('Nouns::Communication', buttons(folders['Nouns/Communication'])),
    folder('Nouns::Events',    buttons(folders['Nouns/Events'])),
    folder('Nouns::Feelings',  buttons(folders['Nouns/Feelings'])),
    folder('Nouns::Food',      buttons(folders['Nouns/Food'])),

    folder('Nouns::Groups',    buttons(folders['Nouns/Group'])),
    folder('Nouns::Locations', buttons(folders['Nouns/Location'])),
    folder('Nouns::Motives',   buttons(folders['Nouns/Motives'])),
    folder('Nouns::Object',    buttons(folders['Nouns/Object'])),
    folder('Nouns::People',    buttons(folders['Nouns/People'])),
    folder('Nouns::Plants',    buttons(folders['Nouns/Plants'])),
    folder('Nouns::Possession',    buttons(folders['Nouns/Possession'])),

    folder('Nouns::Animals::Body Parts', buttons(folders['Nouns/Animals/Body Parts'])),
]
nouns_subfolders_names = [sf['id'] for sf in nouns_subfolders]


verb_categories = ['verb.body', 'verb.change', 'verb.cognition', 'verb.communication', 'verb.competition', 'verb.consumption', 'verb.contact', 'verb.creation', 'verb.emotion', 'verb.motion', 'verb.perception', 'verb.possession', 'verb.social', 'verb.stative', 'verb.weather']
verbs_subfolders = [folder('Verbs::' + category.split('.')[1].capitalize(), buttons(cats(category))) for category in verb_categories]
verbs_subfolders_names = [sf['id'] for sf in verbs_subfolders]

# 9 cols x 6 rows
system = {
    "name": "Wikipedia 2016 Top 10k",
    "description": "A system generated from the top 10,000 words on Wikipedia in 2016.",
    "folders": [
        folder('Pronouns',  buttons(folders['Pronouns']), toplevel=True),
        folder('Nouns',     buttons(nouns_subfolders_names, folder=True), toplevel=True),
        folder('Verbs',     buttons(verbs_subfolders_names, folder=True), toplevel=True),
        folder('Attributes',    buttons(folders['Attributes']), toplevel=True),
        folder('Modifiers',     buttons(folders['Modifiers']), toplevel=True),
        *nouns_subfolders,
        *verbs_subfolders,
    ],
    "hotbar": {
        "rows": 1,
        "cols": 9,
        "buttons": list(map(lambda word: {"label": word},
            """
            the
            be
            to
            of
            and
            a
            in
            have
            for
            not
            on
            with
            as
            at
            but
            by
            from
            or
            out
            if
            """.split())),
    },
    "variants": { # TODO.
        "hello": [
            {"label": "hello"},
            {"label": "Hello!"}
        ],
        "hey": [
            {"label": "hey"},
            {"label": "Hey!"}
        ],
    },
    "related": { # TODO.
        "hello": [
            {"label": "hello"},
            {"label": "hi"},
            {"label": "hey"}
        ],
        "world": [
            {"label": "world"},
            {"label": "planet"}
        ],
    },
}

import json
from pathlib import Path
with open(Path(__file__).parent.with_name("system-wiki2016.json"), "w") as file:
    json.dump(system, file, indent=4)

#['adj.all', 'adj.pert', 'adv.all', 'noun.Tops',  ]
