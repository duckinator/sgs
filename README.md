# Speech Generation System

**WARNING:** This software is NOT ready for everyday use.

If you need AAC software right now, please consider [Proloquo by AssistiveWare](https://www.assistiveware.com/products/proloquo)
or [TD Snap by tobii dynavox](https://us.tobiidynavox.com/pages/td-snap).

Proloquo is what I use regularly, and it serves me well. TD Snap is much
cheaper and far more flexible, but has a steeper learning curve.

---

SGS is open-source **AAC** _(Accessible and Augmentative Communication)_
software, available for free, intended to be used on a computer or a tablet.

_Accessible and Augmentative Communication_ is a term for
communication methods used in place of or in addition to speech.


### Terminology

These are the terms and definitions SGS uses for common AAC concepts:

* **button:** a single interactive element, representing a word/phrase OR an action to perform.
* **folder:** a collection of buttons, with a unique name, plus whether buttons should be spoken immediately or appended.
* **system:** a group of folders, plus the name of the default folder.
* **profile:** various metadata + a system that specifies what buttons are different from the base system

### Design Goals

Guiding principles & design considerations:
- Make it quick to learn.
- Don't require precise inputs.
  - Make things big enough to interact with easily, even if someone has motor control issues (as I do).
  - Avoid scrolling; use pagination instead.
- Avoid nested folders.
- Don't make people have to phrase things differently than if they were speaking, writing/typing, etc.
- Make common words quickly accessible.
- Keep related words close.
- Well-organized grids are your friend.

### Understanding the Interface

General layout:
- Top row: global operations (Speak/Clear/Delete/Share buttons, the text that will be spoken).
- Left column (excluding top button): folder selection.
- Right columns: word variants ("Sleep" may have "sleepy", "sleeping", etc) and related words ("like" may have "admire", "appreciate", "enjoy", etc)
- Bottom row: always-available words (extremely common words like "and", "or", "but", etc)
- Central section: words in for the selected folder.

### Thanks & Acknowledgements

SGS has been strongly influenced by my use of Proloquo and TD Snap.
I have no intent to compete with them. A free product can ever match the
quality of Proloquo or the flexibility of TD Snap, and that's okay.

SGS is meant to fill a niche they inherently can't: freely-available AAC software.

---

SGS is released under the MIT license.
