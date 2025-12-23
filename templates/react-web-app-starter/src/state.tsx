import { proxy } from "valtio";
import { subscribeKey } from "valtio/utils";

export const appState = proxy<{
	theme: "dark" | "light";
}>({
	theme:
		(localStorage.getItem("theme") ??
		window.matchMedia("(prefers-color-scheme: dark)").matches)
			? "dark"
			: "light",
});

subscribeKey(appState, "theme", (newTheme) => {
	localStorage.setItem("theme", newTheme);
});
