'use client';

import { useState, useEffect, useCallback } from 'react';
import type { ClientKey } from '@/types';

const STORAGE_KEY = 'omni_client_keys';

export function useClientKeys() {
  const [keys, setKeys] = useState<ClientKey[]>([]);

  // Load from localStorage on mount
  useEffect(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      try {
        setKeys(JSON.parse(saved));
      } catch {
        // Invalid JSON, ignore
      }
    }
  }, []);

  // Save to localStorage when keys change
  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(keys));
  }, [keys]);

  const addKey = useCallback((key: ClientKey) => {
    setKeys(prev => [...prev, key]);
  }, []);

  const removeKey = useCallback((id: string) => {
    setKeys(prev => prev.filter(k => k.id !== id));
  }, []);

  return { keys, addKey, removeKey };
}
