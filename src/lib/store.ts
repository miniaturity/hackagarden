import { writable } from 'svelte/store';

export interface Project {
    name: string;
    total_seconds: number;
    languages: string[];
    most_recent_heartbeat: string | null;
}

export interface Heartbeat {
    project: string | null;
    language: string | null;
    editor: string | null;
    entity: string | null;
}

export interface UserData {
    username: string;
    streak_days: number;
    projects: Project[];
    latest_heartbeat: Heartbeat | null;
    api_key: string | null;
}

export const userData = writable<UserData | null>(null);