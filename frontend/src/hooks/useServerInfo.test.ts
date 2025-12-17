import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { useServerInfo } from './useServerInfo';

vi.mock('@/lib/api', () => ({
  api: {
    getServerInfo: vi.fn(),
  },
}));

import { api } from '@/lib/api';

describe('useServerInfo', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should fetch server info on mount', async () => {
    const mockServerInfo = {
      server_public_key: 'test-key-123',
      server_name: 'Test Server',
      version: '0.1.0',
    };

    (api.getServerInfo as ReturnType<typeof vi.fn>).mockResolvedValueOnce(mockServerInfo);

    const { result } = renderHook(() => useServerInfo());

    expect(result.current.loading).toBe(true);
    expect(result.current.serverInfo).toBe(null);

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.serverInfo).toEqual(mockServerInfo);
    expect(result.current.error).toBe(null);
  });

  it('should handle fetch error', async () => {
    (api.getServerInfo as ReturnType<typeof vi.fn>).mockRejectedValueOnce(
      new Error('Network error')
    );

    const { result } = renderHook(() => useServerInfo());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.serverInfo).toBe(null);
    expect(result.current.error).toBe('Network error');
  });

  it('should provide refetch function', async () => {
    const mockServerInfo = {
      server_public_key: 'key-1',
      server_name: 'Server 1',
      version: '0.1.0',
    };

    (api.getServerInfo as ReturnType<typeof vi.fn>).mockResolvedValue(mockServerInfo);

    const { result } = renderHook(() => useServerInfo());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    // Call refetch
    result.current.refetch();

    expect(api.getServerInfo).toHaveBeenCalledTimes(2);
  });
});
