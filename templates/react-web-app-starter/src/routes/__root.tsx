import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useSnapshot } from "valtio";
import { appState } from "@/state";

export const Route = createRootRoute({
	component: Root,
	notFoundComponent: NotFoundComponent,
	head: () => ({ links: [{ rel: "icon", href: "/assets/favicon.svg" }] }),
});

function Root() {
	const appSnap = useSnapshot(appState);

	return (
		<div
			className="min-w-svw min-h-svh transition-colors bg-base-100 font-body text-base-content"
			data-theme={appSnap.theme}
		>
			<Outlet />
			<TanStackRouterDevtools position="bottom-right" />
		</div>
	);
}
function NotFoundComponent() {
	return <div>Route Not Found</div>;
}
