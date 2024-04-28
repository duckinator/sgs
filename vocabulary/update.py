#!/usr/bin/env python3

from bs4 import BeautifulSoup
import csv
import itertools
import nltk
from pathlib import Path
from urllib.request import urlopen
import re

# https://wordnet.princeton.edu/
nltk.download("wordnet")

from nltk.corpus import wordnet as wn

CACHE_DIR = Path("cache")
CACHE_DIR.mkdir(exist_ok=True)

def get_freq_list_html():
    freq_list_url = "https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists/English/Wikipedia_(2016)"
    freq_list_html = CACHE_DIR / "wiktionary-freq-list.html"

    if freq_list_html.exists():
        print(f"Using cached {freq_list_html.name}.")
    else:
        print(f"Downloading {freq_list_url} as {str(freq_list_html)}.")
        with urlopen(freq_list_url) as response:
                freq_list_html.write_text(response.read().decode())

    assert freq_list_html.exists()

    return freq_list_html.read_text()


def get_word_list():
    word_list = CACHE_DIR / "word-list.txt"

    soup = BeautifulSoup(get_freq_list_html(), "html.parser")

    print("Scanning for words...")

    # Match all wiki links under a heading.
    links = soup.select(".mw-heading ~ p a[href^='/wiki/']")
    words = [link["title"] for link in links]

    total_original = len(words)
    print(f"  Checking {total_original} words")


    # Remove proper nouns, since those are going to be handled separately.
    words = [word for word in words if re.compile("^[a-z]").match(word)]
    total_without_proper_nouns = len(words)
    print(f"  Removed  {total_original - total_without_proper_nouns} proper nouns.")

    # Remove multi-word phrases like "the same", "as well", etc.
    words = [word for word in words if ' ' not in word]
    total_without_phrases = len(words)
    print(f"  Removed  {total_without_proper_nouns - total_without_phrases} multi-word phrases.")

    # Remove abbreviations (rd, km) and single consonants used as words (t, v).
    words = [word for word in words if not re.compile("^[bcdfghjklmnpqrstvwxz]{1,2}$").match(word.lower())]
    words = [word for word in words if word.lower() not in ["e", "y", "da", "de", "des", "le", "ed"]]
    total_without_abbrs = len(words)
    print(f"  Removed  {total_without_phrases - total_without_abbrs} two-letter abbreviations and single consonants")

    # Remove words abbreviated with a dot.
    words = [word for word in words if "." not in word]
    total_without_abbrs2 = len(words)
    print(f"  Removed  {total_without_abbrs - total_without_abbrs2} words abbreviated with a dot (e.g. \"p.\")")

    print()

    print(f"  Accepted {total_without_phrases} words.")

    word_list.write_text("\n".join(words))
    print(f"Saved word list to {word_list.name}.")

    return word_list.read_text().splitlines()


def get_normalized_word_list():
    def normalize(word):
        chunked_lemma_names = [[x.name() for x in s.lemmas()] for s in wn.synsets(word)]

        # Given [[a, b], [b, c], [b], [c, b]] we want `num_lemma_names`
        # to be {'a': 1, 'b': 4, 'c': 2}.
        num_lemma_names = {}
        for chunk in chunked_lemma_names:
            for name in chunk:
                if not name in num_lemma_names:
                    num_lemma_names[name] = 0
                num_lemma_names[name] += 1

        # Given `num_lemma_names` of {'a': 1, 'b': 4, 'c': 2},
        # we want `results` to be [(4, 'b'), (2, 'c'), (1, 'a')]
        results = sorted(itertools.zip_longest(num_lemma_names.values(), num_lemma_names.keys()), reverse=True)

        # FIXME: Debug printing for known problems.
        # WordNet doesn't have "me", "it", "an", "who" as words. Only acronyms.
        for result in results:
            if result[1] in ['Maine', 'IT', 'AN', 'WHO']:
                print(word, results)

        # Handle situations like [(1, 'information_technology'), (1, 'IT')].
        results = sorted(filter(lambda x: "_" not in x[1], results), reverse=True)

        # Given `results` of [('b', 4), ('c', 2), ('a', 1)],
        # we want `results` to be `['b', 'c', 'a'].
        results = [x[1] for x in results]

        # Given e.g. 'singer' and 'Singer', prefer the all-lowercase variant.
        for result in results:
            if result.lower() in results:
                return result.lower()

        if len(results) > 0:
            return results[0]
        else:
            return word

    normalized_word_list = CACHE_DIR / "normalized-word-list.txt"

    print("Normalizing word list...")
    normalized_words = [normalize(word) for word in get_word_list()]
    normalized_word_list.write_text("\n".join(normalized_words))
    print(f"Saved normalized word list to {normalized_word_list.name}.")

    return normalized_word_list.read_text().splitlines()


def get_word_list_with_syntactic_categories():
    cats = {}
    for word in get_normalized_word_list():
        cats[word] = set([s.lexname() for s in wn.synsets(word) if s.name().startswith(f"{word}.")])
    return cats


def all_syntactic_categories_for(cats):
    all_cats = set()
    for (word, word_cats) in cats.items():
        all_cats = all_cats.union(word_cats)
    return all_cats


def write_words_with_cats_tsv():
    words_with_cats = get_word_list_with_syntactic_categories()
    cats = all_syntactic_categories_for(words_with_cats)
    cols = ["word", *sorted(cats)]


    print("Generating TSV... ", end="")
    with open("words.tsv", "w", newline="") as tsvfile:
        writer = csv.writer(tsvfile, delimiter="\t", lineterminator="\n")

        writer.writerow(cols)
        for (word, cats) in words_with_cats.items():
            #print(f"{word:<20} {cats}")
            row = [""] * len(cols)
            for cat in cats:
                idx = cols.index(cat)
                row[idx] = 1
            row[0] = word
            writer.writerow(row)

    print("Done")

write_words_with_cats_tsv()
