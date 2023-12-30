This is a simple desktop GUI alarm clock for personal use, created to practice my Rust. Currently has only been tested on Ubuntu 22.04, GNOME desktop (Wayland).

# Install Notes
## For egui
[GitHub documentation says](https://github.com/emilk/egui#demo) these libraries are required on linux:
```bash
sudo apt install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```
## For rodio
```cargo build``` will fail without [ALSA (Advanced Linux Sound Architecture)](https://www.alsa-project.org/wiki/Main_Page) installed. To get it to build on my Ubuntu 22.04 system it required I run:
```bash
sudo apt install -y alsa alsa-tools libasound2-dev
```
**Note:** ```alsa``` may be part of ```libasound2-dev```, not 100% sure if ```alsa-tools``` is actually required. I installed the two alsa packages first (and it didn't build), then installed ```libasound2-dev``` and it did build.
