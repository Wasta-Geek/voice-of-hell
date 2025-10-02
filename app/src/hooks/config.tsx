import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { GlobalConfig } from '../bindings/GlobalConfig';

// Hook generator to use for getting config
export function useGetConfig() {
    return useQuery<GlobalConfig>({
        queryFn: () => invoke<GlobalConfig>('get_config'),
        queryKey: [`config`],
    });
};

// Hook generator to use for updating config
export function useUpdateConfig() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: (config: GlobalConfig) => invoke('save_config', { config: config }),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['get_config'] })
        }
    });
};