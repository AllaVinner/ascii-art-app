# ASCII ART APP
[App Link](https://allavinner.github.io/ascii-art-app/)
[md link](README.md)
[[README.md]]
## Intro

## Run
### Option 1
Click on the [App Link](https://allavinner.github.io/ascii-art-app/) and upload an image (it is a static page, so all files remains in your browser).

### Option 2
Requiers npm, but not wasm-pack, cargo or rustc
```
git clone https://github.com/AllaVinner/ascii-art-app.git
npm install
npm run dev
```

## Setup New Project: React + WASM + Vite
Requiers npm, wasm-pack, cargo and rustc.
```
npm create vite@latest
: ascii-art-app
cd ascii-art-app
npm install
cargo generate --git https://github.com/rustwasm/wasm-pack-template
: wasm-src
cd wasm-src
wasm-pack build
cd ..
npm i vite-plugin-wasm
npm i vite-plugin-top-level-await
```

To use the created `wasm-src`, add:
```javascript
//src/App.js
import * as wasm from "../wasm-src/pkg/wasm_src.js";

wasm.greet();
```

To use wasm with Vite, and to be able to publish a Vite app on Github pages, add:
```javascript
//vite.config.js 
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(),
    wasm(),
    topLevelAwait()],
  base: 'my-app' // for github actions
})
```
To be able to deplot a static webpage on github pages, do:
1. Add github action workflow
2. enable github pages on github
   
**ToDo**:
Currently, I have removed the `pkg` from `.gitignore` to not have to compile the rust code in Github Actions. This might be preferable to change.

