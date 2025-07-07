export interface Theme {
	button: React.DetailedHTMLProps<
		React.ButtonHTMLAttributes<HTMLButtonElement>,
		HTMLButtonElement
	>;
}

export interface Addon {
	theme: Theme;
}
