export type SoundEffect =
    { type: "DoNothing" }
    | { type: "PlaySound", file: string }
    | { type: "IncreaseVolume", volume: number }
    | { type: "DecreaseVolume", volume: number }
    | { type: "ClearAllEffects" };

    
export enum SoundEffectType {
    DoNothing = "Do nothing",
    PlaySound = "Play a sound",
    ClearAllEffects = "Clear all effects currently played"
}
