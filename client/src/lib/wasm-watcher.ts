import { exec } from "child_process";
import chokidar from "chokidar";

const WASM_DIR = "./lib/src/**/*";
const BUILD_CMD = "npm run wasm";

console.log("Watching ${WASM_DIR} for changes..");

chokidar.watch(WASM_DIR).on("change", (path: string) => {
	console.log("File changed: ${path}");
	console.log("Rebuilding WASM module..");
	exec(BUILD_CMD, (err, stdout, stderr) => {
		if(err){
			console.error("Error rebuilding WASM: ${stderr}");
			return;
		}
		console.log(stdout);
	});
});
