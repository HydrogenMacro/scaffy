import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chdir } from "node:process";
import { fileURLToPath } from "url";
import { dirname } from "path";
import { readFile } from "node:fs/promises";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

for (const templateRoot of readdirSync(join(__dirname, "../templates"), {
	withFileTypes: true,
})) {
	if (!templateRoot.isDirectory()) continue;
	if (templateRoot.name === "__scaffy_template_contents") continue;
	chdir(join(__dirname, "../templates", templateRoot.name));
	let files = readdirSync(".", { recursive: true, withFileTypes: true });
    let root = { type: "root", children: {} };
	for (const file of files) {
		if (!file.isFile()) continue;
		let parentDirs = file.parentPath.split("/");
		if (parentDirs[0] === ".") parentDirs.shift();
		let current = root.children;
		for (const parentDir of parentDirs) {
			if (!current[parentDir])
				current[parentDir] = {
					type: "folder",
					injectProjectInfo: parentDir.includes("@@SCAFFY_"),
					children: { __proto__: null },
				};
			current = current[parentDir].children;
		}
		current[file.name] = { type: "file", injectProjectInfo: false };
		const fileContents = readFileSync(
			join(
				__dirname,
				"../templates",
				templateRoot.name,
				file.parentPath,
				file.name,
			),
			{ encoding: "utf-8" },
		);
		if (fileContents.includes("@@SCAFFY_"))
			current[file.name].injectProjectInfo = true;
	}
	writeFileSync(
		join(
			__dirname,
			"../templates/__scaffy_template_contents",
			`${templateRoot.name}.json`,
		),
		JSON.stringify(root.children, null, 0),
	);
}
