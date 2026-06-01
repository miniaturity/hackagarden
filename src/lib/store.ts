import { writable } from 'svelte/store';
import type { GardenPlant, GreenhousePlant, OwnedPlant, Plant } from './plants';


export type Project = {
  name: string;
  total_seconds: number;
  languages: { name: string; total_seconds: number }[];
  most_recent_heartbeat: string | null;
};

export type UserData = {
  username: string;
  streak_days: number;
  projects: Project[];
  latest_heartbeat: {
    project: string | null;
    language: string | null;
    editor: string | null;
    entity: string | null;
  } | null;
  api_key: string | null;
};

export const userData = writable<UserData | null>(null);

export type CurrencyState = {
  balance: number;
  total_hours_minted: number;
  language_hours: Record<string, number>;
  total_hours: number;
};

export const currencyState = writable<CurrencyState | null>(null);


export type InventoryState = {
  ownedPlants: OwnedPlant[];
  greenhousePlants: GreenhousePlant[];
  gardenPlants: GardenPlant[];
}

export const inventoryState = writable<InventoryState | null>(null);

