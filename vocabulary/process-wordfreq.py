#!/usr/bin/env python3

import pandas as pd

# pinned = ((7* 3) - 2) most common where PoS==c, pinned a/the?

# Categories:
#   Nouns & Pronouns:      ...
#   Actions:    words that convey action -- bring, walk, run, learn.
#   States:     words that convey state -- be, exist, stand.
#   Attributes: words that describe or define another word.
#   Modifiers:  words that modify another word -- e.g. by specifying manner, place, time, etc.
#   
#   Interjections: 
#   Conjunction: ... (persistent)

# PoS = first letter of https://ucrel.lancs.ac.uk/claws7tags.html
#pos_keys = {
#    'article': 'a', # TODO: Filter out posessive pronouns, somehow.
#    'conjunction': 'c',
#    'determiner': 'd',
#    # don't care about existential-there (e)
#    'proposition': 'i',
#    'adjective': 'j',
#    'number': 'm',
#    'noun': 'n',
#    'pronoun': 'p',
#    'adverb': 'r',
#    # don't care about transitive-marker to (t)
#    'interjection': 'u',
#
#    # what the fuck is V?
#    # V-were, was, being, be (infinitive), am, been, are, is, do (base form/finite), did, doing, do, done, does, have, had, has, "modal auxilliary", "modal catenative", "-s form of lexical verb"
#
#    # don't care about X (not, n't)
#}

cols = [
    "rank",
    "lemma",
    "PoS",
    "freq",
]

with pd.ExcelFile("wordFrequency.xlsx") as xlsx:
    df = pd.read_excel(xlsx, "1 lemmas", usecols=cols)

df.info()

categories = {
    'Nouns & Pronouns': ['n', 'p'], # TODO: are posessive pronouns missing?
    'Verbs': ['v'],
    #'Actions': [''],
    #'States': [''],
    #'Attributes': [''],
    'Modifiers': ['r', 'd', 'i', 'j'],
#    'Interjections': ['u'],
#   Nouns & Pronouns:      ...
#   Actions:    words that convey action -- bring, walk, run, learn.
#   States:     words that convey state -- be, exist, stand.
#   Attributes: words that describe or define another word.
#   Modifiers:  words that modify another word -- e.g. by specifying manner, place, time, etc.
#   
#   Interjections: 
#   Conjunction: ... (persistent)
}


hotbar = []

boards = {}
for (name, pos_filter) in categories.items():
    boards[name] = df[df['PoS'].isin(pos_filter)]['lemma'].tolist()

#print(boards)

for key in boards.keys():
    print(len(boards[key]), "\t", key)
