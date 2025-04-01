//const { exec } = require("child_process");
// const chokidar = require("chokidar");
import { exec } from "child_process";
import chokidar from "chokidar";
import path from "path";
import WebSocket, { WebSocketServer } from "ws";

const WASM_DIR = path.resolve("../lib/src/");
const BUILD_CMD = "concurrently \"npm run wasm\" \"npm run dev\""; // Command to rebuild WASM


console.log(`Watching ${WASM_DIR} for changes...`);


// Start WebSocket Server
const wss = new WebSocketServer({ port: 8080 });

wss.on("connection", function connection(ws) {
	wss.on('error', console.error);
    console.log("⚡ Client connected to WebSocket");
});

const watcher = chokidar.watch(WASM_DIR, {
    ignored: /node_modules/, 
    persistent: true
});


watcher.on("change", (path) => {
    console.log(`File changed: ${path}`);
    console.log("Rebuilding WASM module...");
    
    exec(BUILD_CMD, (err, stdout, stderr) => {
        if (err) {
            console.error(`Error rebuilding WASM: ${stderr}`);
            return;
        }
        console.log(stdout);
    });

	// Notify frontend via WebSocket
    wss.clients.forEach((client) => {
    	if (client.readyState === WebSocket.OPEN) {
		    client.send("reload-wasm");
	    }
    });
});
