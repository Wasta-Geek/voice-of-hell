import { Profile } from "./Profile";

export type AppConfig = {
    last_input_device_used: string | null,
    last_output_device_used: string | null,
    last_profile_index_used: string | null,
    profiles: Array<Profile>,
};
