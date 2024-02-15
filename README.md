# Speech Generation System

**WARNING:** This software is NOT ready for everyday use.

If you need AAC software right now, please consider [Proloquo by AssistiveWare](https://www.assistiveware.com/products/proloquo)
or [TD Snap by tobii dynavox](https://us.tobiidynavox.com/pages/td-snap).

Proloquo is what I use regularly, and it serves me well. TD Snap is much
cheaper and far more flexible, but has a steeper learning curve.

## Building for Windows or Linux

Using the normal `cargo build`/`cargo run` process should be fine.

However, you can also use `x build`/`x run` for consistency with the Android build process.

## Building for Android

This assumes a Debian environment, since that's what I've used for it.

- [rust-mobile/xbuild](https://github.com/rust-mobile/xbuild) (not to be confused with 6 other things called "xbuild")

Clone the repo, and `cd` to the project root:

```
git clone https://github.com/duckinator/sgs.git && cd sgs
```

Then, install required packages:

```
sudo apt install adb android-sdk-platform-tools sdkmanager libssl-dev kotlin curl && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
rustup target install aarch64-linux-android && \
./bin/download-gradle
```

Add the following to your shell configuration:

```
export ANDROID_HOME="$HOME/android-sdk"
export ANDROID_NDK_VERSION="26.2.11394342"
export ANDROID_NDK_HOME="${ANDROID_HOME}/ndk/${ANDROID_NDK_VERSION}"

export PATH="$PATH:$HOME/sgs-gradle/bin"
```

Open a new shell, or apply that configuration manually.

Then, run `sdkmanager --install "ndk;${ANDROID_NDK_VERSION}"`.

<!-- Plug in an Android device you want to use for testing, run `x doctor`, and get the device ID. -->

<!--

Then run:

```
x run --device adb:ID-FROM-X-DOCTOR
```
-->

Then run: (TODO: figure out actual commands)

```
cargo ndk -t arm64-v8a ??? build --release
gradle ???
```

TODO: Figure out how to run on the device.


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
