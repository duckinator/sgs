#!/usr/bin/env python3

from dataclasses import dataclass
from dataclasses_json import dataclass_json
import itertools
from pathlib import Path
from pyexcel_odsr import get_data
from typing import Optional

@dataclass_json
@dataclass
class Button:
    label: str
    parent: Optional[str]
    pronunciation: Optional[str]
    image: Optional[str]
    folder: Optional[str]


@dataclass_json
@dataclass
class Folder:
    name: str
    id: str
    toplevel: bool
    immediate: bool
    rows: int
    cols: int
    buttons: list[Optional[Button]]

@dataclass_json
@dataclass
class Hotbar:
    rows: int
    cols: int
    buttons: list[Optional[Button]]

@dataclass_json
@dataclass
class System:
    name: str
    description: str
    folders: list[Folder]
    hotbar: Hotbar
    variants: dict[str, list[Button]]
    related: dict[str, list[Button]]


def mkbutton(label, parent=None):
    if label == '':
        return None

    if label.startswith("$"):
        ...

    pronunciation = None
    image = None
    folder = None

    # Button(label, parent, pronunciation, image, folder)
    return Button(label, parent, pronunciation, image, folder)


def mkbuttons(buttons, parent=None):
    # Flatten the list.
    buttons = list(itertools.chain(*buttons))
    return [mkbutton(button, parent) for button in buttons]


def mkfolder(name, data):
    toplevel = name.startswith("^")
    immediate = name.startswith("!")
    if toplevel or immediate:
        folder_id = name[1:]
    else:
        folder_id = name
    name = folder_id.split("::")[-1]
    rows = 6
    cols = 9
    buttons = mkbuttons(data, folder_id)

    return Folder(name, folder_id, toplevel, immediate, rows, cols, buttons)


def mkhotbar():
    words = """
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
    """.splitlines()
    words = [word.strip() for word in words]
    return Hotbar(1, 9, mkbuttons(words))


def main():
    file_dir = Path(__file__).parent
    system_ods = (file_dir / '..' / 'system.ods').resolve()

    data = get_data(str(system_ods))

    # This is purely used for documenting the file itself.
    data.pop('HELP')

    related = data.pop('RELATED')
    variants = data.pop('VARIANTS')
    #print(related)
    #print(variants)

    name = "Default System"
    description = "A custom system put together by duckinator"
    folders = [mkfolder(k, data[k]) for k in data.keys()]
    hotbar = mkhotbar()
    variants = {}
    related = {}

    system = System(name, description, folders, hotbar, variants, related)

    (file_dir / '..' / 'system.json').resolve().write_text(system.to_json(indent=4))


if __name__ == "__main__":
    main()
