# chorder

See what chord you're playing on your MIDI controller in real time (amazing!)

## Background

*Why did you make this?*

I often noodle on my MIDI keyboard and don't know the name of the chord I'm playing. Now I can just leave this open on the side to see!

*Shouldn't this just be a feature of the DAW?*

Yes and I just found [this](https://forums.cockos.com/showthread.php?t=263150) (for Reaper), but I guess this is cool if you want this functionality outside of the DAW.

*Why does it have such a dumb name?*

Couldn't tell you.

## Requirements

Since I'm building for Windows, you will need MinGW:

```bash
sudo pacman -S mingw-w64-gcc
```

## Usage

Clone this repository then:

```bash
cargo run
```

As of right now there's no way to pick which MIDI device to use, it will just use the first one (Port 0).
