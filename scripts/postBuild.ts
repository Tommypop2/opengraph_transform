import { writeFile } from "fs/promises";
const packageJson = `{
	"name": "opengraph_transform",
	"version": "0.1.0",
	"type": "module",
	"exports": {
		".": {
			"import": "./opengraph_transform.js",
			"types": "./opengraph_transform.d.ts"
		}
	},
	"files": [
		"opengraph_transform_bg.wasm",
		"opengraph_transform.js",
		"opengraph_transform.d.ts"
	],
	"module": "opengraph_transform.js",
	"types": "opengraph_transform.d.ts",
	"sideEffects": [
		"./snippets/*"
	]
}
`;
(async () => {
	await writeFile("./pkg/package.json", packageJson);
})();
