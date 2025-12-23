import { createFileRoute } from "@tanstack/react-router";

import { cn } from "@/utils/cn";

export const Route = createFileRoute("/app/")({
	component: App,
});

function App() {
	return <div className={cn("w-full h-svh bg-base-200")}>App</div>;
}
