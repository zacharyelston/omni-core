import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, waitFor, act } from '@testing-library/react';
import { useSettings } from './useSettings';

vi.mock('@/lib/api', () => ({
  api: {
    getSettings: vi.fn(),
    updateSettings: vi.fn(),
  },
}));

import { api } from '@/lib/api';

const mockConfig = {
  server: { id: '1', name: 'Test', description: 'Test server', version: '0.1.0' },
  network: { host: '0.0.0.0', port: 8080, public_url: 'http://localhost:8080' },
  auth: { session_ttl_secs: 3600, admin_session_multiplier: 24 },
  federation: { enabled: true, public: true, sync_interval_secs: 3600, max_known_servers: 1000 },
};

describe('useSettings', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should fetch settings on mount', async () => {
    (api.getSettings as ReturnType<typeof vi.fn>).mockResolvedValueOnce(mockConfig);

    const { result } = renderHook(() => useSettings());

    expect(result.current.loading).toBe(true);

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.config).toEqual(mockConfig);
    expect(result.current.error).toBe(null);
  });

  it('should save settings successfully', async () => {
    (api.getSettings as ReturnType<typeof vi.fn>).mockResolvedValueOnce(mockConfig);
    (api.updateSettings as ReturnType<typeof vi.fn>).mockResolvedValueOnce({});

    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    const updatedConfig = { ...mockConfig, server: { ...mockConfig.server, name: 'Updated' } };

    await act(async () => {
      await result.current.save(updatedConfig);
    });

    expect(api.updateSettings).toHaveBeenCalledWith(updatedConfig);
    expect(result.current.saved).toBe(true);
    expect(result.current.config).toEqual(updatedConfig);
  });

  it('should handle save error', async () => {
    (api.getSettings as ReturnType<typeof vi.fn>).mockResolvedValueOnce(mockConfig);
    (api.updateSettings as ReturnType<typeof vi.fn>).mockRejectedValueOnce(
      new Error('Save failed')
    );

    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    await act(async () => {
      await result.current.save(mockConfig);
    });

    expect(result.current.error).toBe('Save failed');
    expect(result.current.saved).toBe(false);
  });

  it('should allow setting config directly', async () => {
    (api.getSettings as ReturnType<typeof vi.fn>).mockResolvedValueOnce(mockConfig);

    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    const newConfig = { ...mockConfig, server: { ...mockConfig.server, name: 'New Name' } };

    act(() => {
      result.current.setConfig(newConfig);
    });

    expect(result.current.config).toEqual(newConfig);
  });
});
