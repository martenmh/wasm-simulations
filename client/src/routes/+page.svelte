<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<script lang="ts">
	import { onMount } from 'svelte';
	let lib: typeof import('lib');

	// 
	onMount(async () => {
		lib = await import('lib');
		await lib.default();
		lib.greet();

		const socket = new WebSocket("ws://localhost:8080/");

		socket.addEventListener("message", (event) => {
            if (event.data === "reload-wasm") {
                console.log("🔁 WASM update detected, reloading...");
				lib = import('lib');
				lib.default();
				lib.greet();
            }
        });

        socket.addEventListener("open", () => console.log("⚡ WebSocket connected"));
	});
</script>

<button on:click={() => {
	const canvas = document.getElementById('drawing');
	const ctx = canvas.getContext('2d');
	const data = lib.get_set(1500, 1500, -0.15, 0.64)
	const imageData = new ImageData(
		Uint8ClampedArray.from(data),
		1500,
		1500,
	);
	ctx.putImageData(imageData, 0, 0);
	console.log(imageData);
}}>Greet</button>
