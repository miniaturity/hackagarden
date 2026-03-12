
export interface GardenPlant extends OwnedPlant {
  gridLocation: { x: number, y: number }; // note: convert to isometric
  attachment?: string; // id of what the plant is attached to.
  inSunlight: boolean;
}

// Plants grow 1.2x faster in the greenhouse than in the garden,
// however you can customize how your garden looks.
export interface GreenhousePlant extends OwnedPlant {
  gridLocation: { x: number, y: number }; // note: convert to isometric
}

export interface OwnedPlant extends Plant {
  fertilized: boolean;

  watered: boolean;

  // # of total hours when bought
  // in order to keep track of growthRate easily.
  // should never be edited
  hoursWhenBought: number; 

  growth: number; // state of growth (arbitrary unit)
  growthRate: number; // ticked at every hour. COMPUTED

  unitGrowthPerStage: number[]; // if it ends early, it uses the last element as the rest.
  growthStage: number; // 1 -> maxGrowthStage
}

export interface Plant {
  texture: string;
  name: string;
  desc: string;

  cost: number;
  
  fertilizeBoost: number; // growth mult to apply if fertilized
  wateredPunishment: number; // growth mult to apply if not watered (0 = stops)
  baseGrowthRate: number;
  typePerGrowth: {
    stage: number;
    type: PlantType;
  }[];
  maxGrowthStage: number; 

  minHours?: number;
  languagePreqs?: {
    name: string;
    hoursMinimum?: number; 
    // if undefined, checks whether player has ever used the language
  }[];
  
  languageBoosters?: {
    name: string;
    increment: number; // every increment hours, apply growth mult.
    growthMult: number;
  }[];
  
}

type PlantType = "potted" | "ground";