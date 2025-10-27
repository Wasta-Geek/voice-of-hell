export type SoundEffect =
    { type: RustSoundEffect.DoNothing }
    | { type: RustSoundEffect.PlaySound, file: string }
    | { type: RustSoundEffect.IncreaseVolume, volume: number }
    | { type: RustSoundEffect.DecreaseVolume, volume: number }
    | { type: RustSoundEffect.ClearAllEffects };


export function generateSoundEffect(type: RustSoundEffect): SoundEffect {
    switch (type) {
        case RustSoundEffect.IncreaseVolume:
        case RustSoundEffect.DecreaseVolume:
            return { type: type, volume: 50 };
        case RustSoundEffect.PlaySound:
            return { type: type, file: "" };
        default:
            return { type: type };
    }
}

export const SoundEffectType: Record<RustSoundEffect, string> = {
    DoNothing: "Do nothing",
    PlaySound: "Play a sound",
    ClearAllEffects: "Clear all effects",
    IncreaseVolume: "IncreaseVolume",
    DecreaseVolume: "DecreaseVolume"
}

export enum RustSoundEffect {
    DoNothing = "DoNothing",
    PlaySound = "PlaySound",
    ClearAllEffects = "ClearAllEffects",
    IncreaseVolume = "IncreaseVolume",
    DecreaseVolume = "DecreaseVolume"
};