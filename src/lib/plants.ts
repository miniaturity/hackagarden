
export interface GardenPlant extends OwnedPlant {
  gridLocation: { x: number, y: number }; 
  attachment?: string; // id of what the plant is attached to, if any
  inSunlight: boolean;
}

// Plants grow 1.2x faster in the greenhouse than in the garden,
// however you can customize how your garden looks.
export interface GreenhousePlant extends OwnedPlant {
  gridLocation: { x: number, y: number }; 
}

export interface OwnedPlant extends Plant {
  fertilized: boolean;
  watered: boolean;

  // # of total hours when bought
  // in order to keep track of growthRate easily.
  // NOTE: should never be edited after purchase
  readonly hoursWhenBought: number; 

  growth: number; // state of growth (arbitrary unit)
  growthRate: number; // ticked at every hour. COMPUTED

  unitGrowthPerStage: number[]; // if it ends early, it uses the last element as the rest.
  growthStage: number; // 1-indexed
}

export interface Plant extends Item {
  id: number;
  texture: string;
  name: string;
  desc: string;

  cost: number;
  
  fertilizeBoost: number; // growth mult to apply if fertilized
  wateredPunishment: number; // growth mult to apply if not watered (0 = stops)
  baseGrowthRate: number; 
  typePerGrowth: {
    texture: string;
    stage: number;
  }[]; // length = max growth stage

  minHours?: number;
  languagePreqs?: {
    name: string;
    hoursMinimum: number; 
  }[];
  
  languageBoosters?: {
    name: string;
    increment: number; // every [increment] hours, apply growth mult.
    growthMult: number;
  }[];
  
}

export interface Fertilizer extends Item {
  name: string;
  cost: number;
  mult: number;
}


export interface Item {
  readonly texture: string; // path (static)
  readonly float: number; // rarity
}