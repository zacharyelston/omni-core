import { describe, it, expect, vi, beforeEach } from 'vitest';
import { api } from './api';

describe('OmniAPI', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getServerInfo', () => {
    it('should fetch server info successfully', async () => {
      const mockResponse = {
        server_public_key: 'abc123',
        server_name: 'Test Server',
        version: '0.1.0',
      };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      });

      const result = await api.getServerInfo();

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/server/info', {
        headers: { 'Content-Type': 'application/json' },
      });
      expect(result).toEqual(mockResponse);
    });

    it('should throw error on failed request', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
        status: 500,
        json: async () => ({ message: 'Server error' }),
      });

      await expect(api.getServerInfo()).rejects.toThrow('Server error');
    });
  });

  describe('getSettings', () => {
    it('should fetch settings successfully', async () => {
      const mockSettings = {
        server: { id: '1', name: 'Test', description: '', version: '0.1.0' },
        network: { host: '0.0.0.0', port: 8080, public_url: '' },
        auth: { session_ttl_secs: 3600, admin_session_multiplier: 24 },
        federation: { enabled: true, public: true, sync_interval_secs: 3600, max_known_servers: 1000 },
      };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockSettings,
      });

      const result = await api.getSettings();

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/settings', {
        headers: { 'Content-Type': 'application/json' },
      });
      expect(result).toEqual(mockSettings);
    });
  });

  describe('updateSettings', () => {
    it('should update settings successfully', async () => {
      const mockSettings = {
        server: { id: '1', name: 'Updated', description: '', version: '0.1.0' },
        network: { host: '0.0.0.0', port: 8080, public_url: '' },
        auth: { session_ttl_secs: 7200, admin_session_multiplier: 24 },
        federation: { enabled: true, public: true, sync_interval_secs: 3600, max_known_servers: 1000 },
      };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => ({}),
      });

      await api.updateSettings(mockSettings);

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/settings', {
        method: 'PUT',
        body: JSON.stringify(mockSettings),
        headers: { 'Content-Type': 'application/json' },
      });
    });
  });

  describe('registerInit', () => {
    it('should initialize registration successfully', async () => {
      const mockResponse = {
        client_id: 'test-client',
        server_public_key: 'server-key-123',
      };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      });

      const result = await api.registerInit('test-client');

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/register/init', {
        method: 'POST',
        body: JSON.stringify({ client_id: 'test-client' }),
        headers: { 'Content-Type': 'application/json' },
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe('adminLogin', () => {
    it('should login admin successfully', async () => {
      const mockResponse = { message: 'Login successful' };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      });

      const result = await api.adminLogin('admin-key-123');

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/admin/login', {
        method: 'POST',
        body: JSON.stringify({ admin_key: 'admin-key-123' }),
        headers: { 'Content-Type': 'application/json' },
      });
      expect(result).toEqual(mockResponse);
    });

    it('should throw on invalid admin key', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
        status: 401,
        json: async () => ({ message: 'Invalid admin key' }),
      });

      await expect(api.adminLogin('wrong-key')).rejects.toThrow('Invalid admin key');
    });
  });

  describe('getKnownServers', () => {
    it('should fetch known servers successfully', async () => {
      const mockServers = [
        { server_id: '1', name: 'Server 1', public_url: 'http://s1', public_key: 'key1', is_authenticated: true },
        { server_id: '2', name: 'Server 2', public_url: 'http://s2', public_key: 'key2', is_authenticated: false },
      ];

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockServers,
      });

      const result = await api.getKnownServers();

      expect(global.fetch).toHaveBeenCalledWith('/api/v1/servers/all', {
        headers: { 'Content-Type': 'application/json' },
      });
      expect(result).toEqual(mockServers);
    });
  });
});
