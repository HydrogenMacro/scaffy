import { LucideMoon, LucideSun } from "lucide-react";
import { useSnapshot } from "valtio";
import { appState } from "@/state";
import { cn } from "@/utils/cn";

export function Header({ className }: { className: string }) {
	return (
		<div
			className={cn(
				"p-4 w-full h-16 flex justify-between items-center absolute",
				className,
			)}
		>
			<div className="flex gap-2 items-center">
				@@SCAFFY_PROJECT_NAME_TITLECASE@@
			</div>
			<div className="flex gap-2 items-center">
				<ThemeSwitchButton />
			</div>
		</div>
	);
}

export function ThemeSwitchButton() {
	const appSnap = useSnapshot(appState);
	const styling =
		"p-2 rounded-full bg-primary-content/20 hover:bg-primary-content/40 flex-none";
	switch (appSnap.theme) {
		case "dark":
			return (
				<button
					className={styling}
					type="button"
					onClick={() => {
						appState.theme = "light";
					}}
				>
					<LucideMoon className="w-4 h-4" />
				</button>
			);
		case "light":
			return (
				<button
					className={styling}
					type="button"
					onClick={() => {
						appState.theme = "dark";
					}}
				>
					<LucideSun className="w-4 h-4" />
				</button>
			);
	}
}
