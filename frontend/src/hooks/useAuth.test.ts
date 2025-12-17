import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useAuth } from './useAuth';

vi.mock('@/lib/api', () => ({
  api: {
    adminLogin: vi.fn(),
  },
}));

import { api } from '@/lib/api';

describe('useAuth', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should start with isAdmin false', () => {
    const { result } = renderHook(() => useAuth());

    expect(result.current.isAdmin).toBe(false);
    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBe(null);
  });

  it('should login successfully', async () => {
    (api.adminLogin as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      message: 'Login successful',
    });

    const { result } = renderHook(() => useAuth());

    let success: boolean = false;
    await act(async () => {
      success = await result.current.login('valid-admin-key');
    });

    expect(success).toBe(true);
    expect(result.current.isAdmin).toBe(true);
    expect(result.current.error).toBe(null);
  });

  it('should handle login failure', async () => {
    (api.adminLogin as ReturnType<typeof vi.fn>).mockRejectedValueOnce(
      new Error('Invalid admin key')
    );

    const { result } = renderHook(() => useAuth());

    let success: boolean = true;
    await act(async () => {
      success = await result.current.login('wrong-key');
    });

    expect(success).toBe(false);
    expect(result.current.isAdmin).toBe(false);
    expect(result.current.error).toBe('Invalid admin key');
  });

  it('should logout successfully', async () => {
    (api.adminLogin as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      message: 'Login successful',
    });

    const { result } = renderHook(() => useAuth());

    // Login first
    await act(async () => {
      await result.current.login('valid-key');
    });

    expect(result.current.isAdmin).toBe(true);

    // Then logout
    act(() => {
      result.current.logout();
    });

    expect(result.current.isAdmin).toBe(false);
  });
});
