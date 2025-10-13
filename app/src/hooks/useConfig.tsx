import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { AppConfig } from '../types/AppConfig';

// Hook generator to use for getting config
export function useGetConfig() {
    return useQuery<AppConfig>({
        queryFn: () => invoke<AppConfig>('get_config'),
        queryKey: [`config`],
    });
};

// Hook generator to use for updating config
export function useUpdateConfig() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: (config: AppConfig) => invoke('save_config', { config: config }),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['get_config'] })
        }
    });
};