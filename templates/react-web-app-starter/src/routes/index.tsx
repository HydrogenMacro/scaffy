import { createFileRoute } from "@tanstack/react-router";
import { LucideChevronRight } from "lucide-react";
import "overlayscrollbars/overlayscrollbars.css";

import { LinkButton } from "@/components/Button";
import { Header } from "@/components/Header";
import { cn } from "@/utils/cn";
import {
	useOverlayScrollbars,
} from "overlayscrollbars-react";
import { useEffect } from "react";
import { useSnapshot } from "valtio";
import { appState } from "@/state";

export const Route = createFileRoute("/")({
	component: Root,
});

function Root() {
	const appSnap = useSnapshot(appState);
	const [initBodyScrollbars, bodyScrollbarsInstance] = useOverlayScrollbars({
		defer: true,
		options: {
			scrollbars: {
				theme: appSnap.theme === "light" ? "os-theme-dark" : "os-theme-light",
			},
		},
	});
	useEffect(() => {
		initBodyScrollbars(document.body);
		return () => {
			bodyScrollbarsInstance()?.destroy();
		};
	}, [initBodyScrollbars, bodyScrollbarsInstance]);
	return (
		<div className="w-dvw flex flex-col items-stretch overflow-x-hidden">
			<Header className={cn("bg-transparent text-primary-content")}></Header>
			<div className="h-120 bg-primary text-primary-content transition-colors flex flex-col items-center justify-center gap-4">
				<h1 className="font-header text-4xl">
					Lorem ipsum dolor sit amet consectetur.
				</h1>
				<p className="text-1xl">
					Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
					nisi ut aliquip ex ea commodo consequat.
				</p>
				<LinkButton
					href="/app"
					size="normal"
					className="transition-colors bg-accent text-accent-content"
				>
					Open App
					<LucideChevronRight
						size={32}
						strokeWidth={1.5}
						className="group-hover/button:translate-x-1 transition-transform"
					/>
				</LinkButton>
			</div>
			<div className="p-8 flex h-svh transition-colors bg-base-100">
				<div className="flex flex-col flex-1">
					<h2 className="text-2xl">Lorem ipsum dolor sit amet consectetur.</h2>
					<p>
						Excepteur sint occaecat cupidatat non proident, sunt in culpa qui
						officia deserunt mollit anim id est laborum.
					</p>
				</div>
				<div className="flex-1"></div>
			</div>
			<div className="p-4 flex justify-between transition-colors bg-base-100 text-base-900">
				<div className="text-md">
					Copyright &copy;{new Date().getFullYear()} John Doe. All Rights
					Reserved.
				</div>
				<div className="flex gap-2">
					<a href="/#privacy-policy" className="underline">
						Privacy Policy
					</a>
					<a href="/#tos" className="underline">
						Terms of Use
					</a>
				</div>
			</div>
		</div>
	);
}
