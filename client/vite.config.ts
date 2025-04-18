import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [sveltekit(), wasmPack('../lib')],
	server: { watch: { ignored: ["!lib/pkg/**"] }}
});
