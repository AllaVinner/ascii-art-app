# React + Vite + Wasm-Pack

>>npm create vite@latest
: app1
>>cd app1
>>npm install
>>cargo generate --git https://github.com/rustwasm/wasm-pack-template
:wasm-src
>>cd wasm-src
>>wasm-pack build
>>cd ..
>>npm i vite-plugin-wasm
>>npm i vite-plugin-top-level-await
?Include the wasm binaries (add pkg folder), or build them in github somewhere
* Add github action workflow
* enable github pages on github


/////////////////
#src/App.js
import * as wasm from "../wasm-src/pkg/wasm_src.js";

wasm.greet();
/////////////////

/////////////////
#vite.config.js
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
//////////////////