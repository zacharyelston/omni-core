'use client';

import { useState, useEffect, useCallback } from 'react';
import type { KnownServer } from '@/types';
import { api } from '@/lib/api';

export function useKnownServers() {
  const [servers, setServers] = useState<KnownServer[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await api.getKnownServers();
      setServers(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to fetch servers');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { servers, loading, error, refetch: fetch };
}
