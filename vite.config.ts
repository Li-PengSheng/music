import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
export default defineConfig({plugins:[svelte(),tailwindcss()],clearScreen:false,server:{strictPort:true,port:1420},envPrefix:['VITE_','TAURI_ENV_']});
