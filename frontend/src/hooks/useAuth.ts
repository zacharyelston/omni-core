'use client';

import { useState, useCallback } from 'react';
import { api } from '@/lib/api';

export function useAuth() {
  const [isAdmin, setIsAdmin] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const login = useCallback(async (adminKey: string) => {
    setLoading(true);
    setError(null);
    try {
      await api.adminLogin(adminKey);
      setIsAdmin(true);
      return true;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Login failed');
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  const logout = useCallback(() => {
    setIsAdmin(false);
    localStorage.removeItem('omni_api_key');
  }, []);

  return { isAdmin, loading, error, login, logout, setError };
}
