<script lang="ts">
	import "../app.css";
  	import { Alert } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	let lib: typeof import('lib');
	function generateSet(map, height, width){
		const canvas = document.getElementById('drawing');
		const ctx = canvas.getContext('2d');

		// WASM Magical call
		const data = lib.get_set(height, width, -0.72, -0.22);
		
		const imageData = new ImageData(
			Uint8ClampedArray.from(data),
			height,
			width,
		);
		ctx.putImageData(imageData, 0, 0);

		// put in leaflet map
		const dataURL = canvas.toDataURL();
		const bounds = [[0,0], [800, 800]];
		L.imageOverlay(dataURL, bounds).addTo(map);

	}
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

		const height = 1800;
		const width = 1800;
		var map = L.map('map', {crs: L.CRS.Simple}).setView([400, 400], 1);

		// Create MandelBrot set
		generateSet(map, height, width);


		// TODO: Might have to do without end
		map.on('zoomend', () => {
			console.log("zoom switch");
			const zoomLevel = map.getZoom();
			const mapBounds = map.getBounds();
			console.log(mapBounds);
			console.log("zoom level: " + zoomLevel);
			generateSet(map, height * zoomLevel, width * zoomLevel);
		});
		map.on('moveend', () => {
			console.log("move switch");
		});


		// Ideas: bunch of fractals
		// show grid
		// show the iterative process of complex value C under mouse
		// show julia set corresponding to value C under mouse
	});

</script>

<div style="height: 1270px;" id="map"></div>
<canvas id='drawing' style="display: none;" width="1800" height="1800"></canvas>
