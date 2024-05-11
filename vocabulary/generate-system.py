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

import csv

with open("words.tsv") as tsvfile:
    reader = csv.reader(tsvfile, delimiter="\t")
    headers = next(reader)

    words = dict(map(lambda x: [x, set()], headers[1:]))
    for row in reader:
        word = row[0]
        for idx in range(1, len(headers)):
            group = headers[idx]
            in_group = bool(row[idx])
            if in_group:
                words[group].add(word)

def cats(*folders):
    global words # i know, i know, i'm sorry
    results = set()
    for category in folders:
        results.update(words[category])
    return results

folders = {
    'Nouns/Actions': cats('noun.act'),
    'Nouns/Artifacts': cats('noun.artifact'),
    'Nouns/Attributes': cats('noun.attribute'),
    'Body': cats('noun.body'),
    'Nouns/Cognition': cats('noun.cognition'),
    'Nouns/Communication': cats('noun.communication').difference({'a', 'u'}),
    'Events': cats('noun.event'),
    'Feelings': cats('noun.feeling'),
    'Food': cats('noun.food'),
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

folders['Pronouns'] = {
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
}

folders['Animals/Body Parts'] = {
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
}

folders['Animals'] = cats('noun.animal').difference(folders['Animals/Body Parts'], {
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
    folders['Body'],
    folders['Nouns/Cognition'],
    folders['Nouns/Communication'],
    folders['Events'],
    folders['Feelings'],
    folders['Food'],
    folders['Nouns/Group'],
    folders['Nouns/Location'],
    folders['Nouns/Motives'],
    folders['Nouns/Object'],
    folders['Nouns/People'],
    folders['Nouns/Plants'],
    folders['Nouns/Possession'],
    folders['Animals/Body Parts'],
    folders['Animals'],
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


def folder(name, buttons, toplevel=False, immediate=False, rows=6, cols=9):
    return {
        "name": name,
        "toplevel": toplevel,
        "immediate": immediate,
        "rows": rows,
        "cols": cols,
        "buttons": buttons,
    }

def button(word, **options):
    return {'label': word, **options}

def buttons(words, **options):
    remaining = []
    if options.get('folder', False):
        remaining = [None] * ((6 * 9) - len(words))
    return [button(word, **options) for word in words] + remaining

def folder_button(word, **options):
    return button(word, folder=True, **options)

nouns_subfolders = [
    folder('Animals',   buttons(['Animal Body Parts'], folder=True) + buttons(folders['Animals'])),
    folder('Body',      buttons(folders['Body'])),
    folder('Actions',   buttons(folders['Nouns/Actions'])),
    folder('Artifacts', buttons(folders['Nouns/Artifacts'])),
    folder('Cognition', buttons(folders['Nouns/Cognition'])),
    folder('Communication', buttons(folders['Nouns/Communication'])),
    folder('Events',    buttons(folders['Events'])),
    folder('Feelings',  buttons(folders['Feelings'])),
    folder('Food',      buttons(folders['Food'])),

    folder('Groups',    buttons(folders['Nouns/Group'])),
    folder('Locations', buttons(folders['Nouns/Location'])),
    folder('Motives',   buttons(folders['Nouns/Motives'])),
    folder('Object',    buttons(folders['Nouns/Object'])),
    folder('People',    buttons(folders['Nouns/People'])),
    folder('Plants',    buttons(folders['Nouns/Plants'])),
    folder('Possession',    buttons(folders['Nouns/Possession'])),

    folder('Animal Body Parts', buttons(folders['Animals/Body Parts'])),
]

nouns_subfolders_names = [
    'Animals',
    'Body',
    'Actions',
    'Artifacts',
    'Cognition',
    'Communication',
    'Events',
    'Feelings',
    'Food',

    'Group',
    'Location',
    'Motives',
    'Object',
    'People',
    'Plants',
    'Possession',
]


verb_categories = ['verb.body', 'verb.change', 'verb.cognition', 'verb.communication', 'verb.competition', 'verb.consumption', 'verb.contact', 'verb.creation', 'verb.emotion', 'verb.motion', 'verb.perception', 'verb.possession', 'verb.social', 'verb.stative', 'verb.weather']
verbs_subfolders = [folder(category.split('.')[1].capitalize(), buttons(cats(category))) for category in verb_categories]
verbs_subfolders_names = [sf['name'] for sf in verbs_subfolders]

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
        "buttons": [], # TODO.
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
