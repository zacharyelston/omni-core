// Server types
export type ServerInfo = {
  server_public_key: string;
  server_name: string;
  version: string;
};

export type ServerSettings = {
  id: string;
  name: string;
  description: string;
  version: string;
};

export type NetworkSettings = {
  host: string;
  port: number;
  public_url: string;
};

export type AuthSettings = {
  session_ttl_secs: number;
  admin_session_multiplier: number;
};

export type FederationSettings = {
  enabled: boolean;
  public: boolean;
  sync_interval_secs: number;
  max_known_servers: number;
};

export type ServerConfig = {
  server: ServerSettings;
  network: NetworkSettings;
  auth: AuthSettings;
  federation: FederationSettings;
};

export type KnownServer = {
  server_id: string;
  name: string;
  public_url: string;
  public_key: string;
  is_authenticated: boolean;
};

// Client types
export type Session = {
  session_id: string;
  api_key: string;
  expires_at: string;
};

export type ClientKey = {
  id: string;
  publicKey: string;
  privateKey: string;
  createdAt: string;
};

export type ServerKey = {
  clientId: string;
  publicKey: string;
  registeredAt?: string;
};

// Tab type
export type Tab = 'home' | 'register' | 'client-keys' | 'server-keys' | 'settings';

// API Response types
export type RegisterInitResponse = {
  client_id: string;
  server_public_key: string;
};

export type RegisterCompleteResponse = {
  api_key: string;
  message: string;
};

export type ServerKeysResponse = {
  keys: Array<{
    client_id: string;
    public_key: string;
  }>;
};
