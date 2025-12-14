'use client';

import { useState, useEffect, useCallback } from 'react';
import type { ServerConfig } from '@/types';
import { api } from '@/lib/api';

export function useSettings() {
  const [config, setConfig] = useState<ServerConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  const fetch = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await api.getSettings();
      setConfig(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to fetch settings');
    } finally {
      setLoading(false);
    }
  }, []);

  const save = useCallback(async (newConfig: ServerConfig) => {
    setSaving(true);
    setError(null);
    try {
      await api.updateSettings(newConfig);
      setConfig(newConfig);
      setSaved(true);
      setTimeout(() => setSaved(false), 3000);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to save settings');
    } finally {
      setSaving(false);
    }
  }, []);

  useEffect(() => {
    fetch();
  }, [fetch]);

  return { config, setConfig, loading, saving, error, saved, refetch: fetch, save };
}
