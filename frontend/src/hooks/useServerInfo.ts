'use client';

import { useState, useEffect, useCallback } from 'react';
import type { ServerInfo } from '@/types';
import { api } from '@/lib/api';

export function useServerInfo() {
  const [serverInfo, setServerInfo] = useState<ServerInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await api.getServerInfo();
      setServerInfo(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to fetch server info');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { serverInfo, loading, error, refetch: fetch };
}
