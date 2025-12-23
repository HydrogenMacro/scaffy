import type { PropsWithChildren } from "react";
import { cn } from "@/utils/cn";

type BtnSizes = "compact" | "normal";
export function Button({
	children = "Button",
	size,
	onClick,
	className,
}: PropsWithChildren<{
	size: BtnSizes;
	onClick?: () => void;
	className: string;
}>) {
	return (
		<button
			className={cn(
				"text-center leading-normal flex items-center relative group/button",
				size === "compact" && "p-2 rounded-md text-md",
				size === "normal" && "p-3 rounded-md text-xl",
				className,
			)}
			onClick={onClick}
			type="button"
		>
			{children}
		</button>
	);
}
export function LinkButton({
	children = "Button",
	href,
	size,
	onClick,
	className,
}: PropsWithChildren<{
	href: string;
	size: BtnSizes;
	onClick?: () => void;
	className: string;
}>) {
	return (
		<a
			className={cn(
				"group/button",
				size === "compact" && "p-2 rounded-md text-md",
				size === "normal" && "p-3 rounded-md text-xl",
				className,
			)}
			onClick={onClick}
			type="button"
			href={href}
		>
			<div
				className={cn(
					"text-center leading-normal flex items-center relative group-hover/button:underline",
					size === "compact" && "before:h-0.25 gap-1",
					size === "normal" && "before:h-0.25 gap-2",
				)}
			>
				{children}
			</div>
		</a>
	);
}
