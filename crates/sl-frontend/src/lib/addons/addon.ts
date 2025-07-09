export interface Addon {
    name: string;
    author?: string;
    version?: string;

    /* Addon Lifecycle */
    onLoad: () => void;
    onTick: (delta: number) => void;
    onUnload: () => void;

    commands?: Record<string, (...args: any[]) => any>;
    state?: Record<string, any>;
}