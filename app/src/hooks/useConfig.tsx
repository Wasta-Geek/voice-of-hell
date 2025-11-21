import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

import { AppConfig } from '@/types/AppConfig';

// Hook to use for managing config
export function useConfig() {
    const queryClient = useQueryClient();
    const configMutation = useMutation({
        mutationFn: (config: AppConfig) => invoke<AppConfig>('save_config', { config: config }),
        onSuccess: (config: AppConfig) => {
            console.log("CONFIG RECEIVED:")
            console.log(config)
            queryClient.setQueryData(['config'], config)
        },
        onError: (error: Error, vara: AppConfig) => {
            console.log("??????????????")
        }
    });

    const { data } = useQuery<AppConfig>({
        queryFn: () => invoke<AppConfig>('get_config'),
        queryKey: ['config']
    });

    return [data, configMutation.mutateAsync] as [AppConfig, typeof configMutation.mutateAsync];
};