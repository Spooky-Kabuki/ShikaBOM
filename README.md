# ShikaBOM
This project replaces BOMIST as a full ERP system really

## Prereqs
See: https://tauri.app/v1/guides/getting-started/prerequisites
### MacOS
1. Have x-code installed
2. Have rust installed
3. Have npm/node installed

### Arch Linux
1. Install prereqs
```shell
sudo pacman -Syu
sudo pacman -S --needed \
webkit2gtk \
base-devel \
curl \
wget \
openssl \
appmenu-gtk-module \
gtk3 \
libappindicator-gtk3 \
librsvg \
libvips
```

2. Install rust
3. Install node/npm

## Quick Start Setup
This assumes you have this repo that you're ready to run

1. ``npm install --save-dev @sveltejs/adapter-static@next``
2. ``cargo install tauri-cli``

## Run the project
``cargo tauri dev``
Live updates should happen after this, no need to re-run unless you've stopped the app.
The Jetbrains Fleet build config is already configured for the build command.
