import type {
  ServerInfo,
  ServerConfig,
  KnownServer,
  RegisterInitResponse,
  RegisterCompleteResponse,
  ServerKeysResponse,
} from '@/types';

class OmniAPI {
  private baseUrl: string;

  constructor(baseUrl = '/api/v1') {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options?: RequestInit
  ): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Request failed' }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return response.json();
  }

  // Server Info
  async getServerInfo(): Promise<ServerInfo> {
    return this.request<ServerInfo>('/server/info');
  }

  // Settings
  async getSettings(): Promise<ServerConfig> {
    return this.request<ServerConfig>('/settings');
  }

  async updateSettings(config: ServerConfig): Promise<void> {
    await this.request('/settings', {
      method: 'PUT',
      body: JSON.stringify(config),
    });
  }

  // Registration
  async registerInit(clientId: string): Promise<RegisterInitResponse> {
    return this.request<RegisterInitResponse>('/register/init', {
      method: 'POST',
      body: JSON.stringify({ client_id: clientId }),
    });
  }

  async registerComplete(
    clientId: string,
    encryptedPublicKey: { nonce: string; ciphertext: string }
  ): Promise<RegisterCompleteResponse> {
    return this.request<RegisterCompleteResponse>('/register/complete', {
      method: 'POST',
      body: JSON.stringify({
        client_id: clientId,
        encrypted_client_public_key: encryptedPublicKey,
      }),
    });
  }

  async getServerKeys(): Promise<ServerKeysResponse> {
    return this.request<ServerKeysResponse>('/register/keys');
  }

  // Admin
  async adminLogin(adminKey: string): Promise<{ message: string }> {
    return this.request<{ message: string }>('/admin/login', {
      method: 'POST',
      body: JSON.stringify({ admin_key: adminKey }),
    });
  }

  // Servers (Federation)
  async getKnownServers(): Promise<KnownServer[]> {
    return this.request<KnownServer[]>('/servers/all');
  }
}

export const api = new OmniAPI();
export default api;
