'use client';

import { useState, useEffect } from 'react';
import { QRCodeSVG } from 'qrcode.react';

interface Session {
  session_id: string;
  api_key: string;
  expires_at: string;
}

interface ClientKey {
  id: string;
  publicKey: string;
  privateKey: string;
  createdAt: string;
}

interface ServerKey {
  clientId: string;
  publicKey: string;
  registeredAt?: string;
}

interface ServerInfo {
  server_public_key: string;
  server_name: string;
  version: string;
}

interface KnownServer {
  server_id: string;
  name: string;
  public_url: string;
  public_key: string;
  is_authenticated: boolean;
}

type Tab = 'home' | 'register' | 'client-keys' | 'server-keys' | 'settings';

interface ServerConfig {
  server: {
    id: string;
    name: string;
    description: string;
    version: string;
  };
  network: {
    host: string;
    port: number;
    public_url: string;
  };
  auth: {
    session_ttl_secs: number;
    admin_session_multiplier: number;
  };
  federation: {
    enabled: boolean;
    public: boolean;
    sync_interval_secs: number;
    max_known_servers: number;
  };
}

export default function Home() {
  const [activeTab, setActiveTab] = useState<Tab>('home');
  const [session, setSession] = useState<Session | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [clientId, setClientId] = useState('');
  const [clientKeys, setClientKeys] = useState<ClientKey[]>([]);
  const [serverKeys, setServerKeys] = useState<ServerKey[]>([]);
  const [pendingServerKey, setPendingServerKey] = useState<string | null>(null);
  const [serverInfo, setServerInfo] = useState<ServerInfo | null>(null);
  const [adminKey, setAdminKey] = useState('');
  const [isAdmin, setIsAdmin] = useState(false);
  const [showMyQR, setShowMyQR] = useState(false);
  const [knownServers, setKnownServers] = useState<KnownServer[]>([]);
  const [serverConfig, setServerConfig] = useState<ServerConfig | null>(null);
  const [settingsSaved, setSettingsSaved] = useState(false);

  // Load saved keys and server info on mount
  useEffect(() => {
    const saved = localStorage.getItem('omni_client_keys');
    if (saved) {
      setClientKeys(JSON.parse(saved));
    }
    fetchServerKeys();
    fetchServerInfo();
    fetchKnownServers();
    fetchSettings();
  }, []);

  const fetchSettings = async () => {
    try {
      const res = await fetch('/api/v1/settings');
      if (res.ok) {
        const data = await res.json();
        setServerConfig(data);
      }
    } catch {
      // Ignore errors
    }
  };

  const saveSettings = async () => {
    if (!serverConfig) return;
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/settings', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(serverConfig),
      });
      if (!res.ok) throw new Error('Failed to save settings');
      setSettingsSaved(true);
      setTimeout(() => setSettingsSaved(false), 3000);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to save');
    } finally {
      setLoading(false);
    }
  };

  const fetchKnownServers = async () => {
    try {
      const res = await fetch('/api/v1/servers/all');
      if (res.ok) {
        const data = await res.json();
        setKnownServers(data);
      }
    } catch {
      // Ignore errors
    }
  };

  const fetchServerInfo = async () => {
    try {
      const res = await fetch('/api/v1/server/info');
      if (res.ok) {
        const data = await res.json();
        setServerInfo(data);
      }
    } catch {
      // Ignore errors
    }
  };

  // Save client keys when they change
  useEffect(() => {
    localStorage.setItem('omni_client_keys', JSON.stringify(clientKeys));
  }, [clientKeys]);

  const fetchServerKeys = async () => {
    try {
      const res = await fetch('/api/v1/register/keys');
      if (res.ok) {
        const data = await res.json();
        setServerKeys(data.keys.map((k: { client_id: string; public_key: string }) => ({
          clientId: k.client_id,
          publicKey: k.public_key,
        })));
      }
    } catch {
      // Ignore errors on initial load
    }
  };

  // Generate a simple keypair (in production, use Web Crypto API)
  const generateKeyPair = (): { publicKey: string; privateKey: string } => {
    const bytes = new Uint8Array(32);
    crypto.getRandomValues(bytes);
    const privateKey = Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
    // In a real implementation, derive public key from private key using X25519
    // For demo, we'll use a placeholder
    const publicBytes = new Uint8Array(32);
    crypto.getRandomValues(publicBytes);
    const publicKey = Array.from(publicBytes).map(b => b.toString(16).padStart(2, '0')).join('');
    return { publicKey, privateKey };
  };

  const handleRegisterInit = async () => {
    if (!clientId.trim()) {
      setError('Please enter a client ID');
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/register/init', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ client_id: clientId }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.message || 'Registration failed');
      
      setPendingServerKey(data.server_public_key);
      
      // Add to server keys list
      setServerKeys(prev => [...prev, {
        clientId: data.client_id,
        publicKey: data.server_public_key,
      }]);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleRegisterComplete = async () => {
    if (!pendingServerKey) {
      setError('No pending registration');
      return;
    }
    setLoading(true);
    setError(null);
    try {
      // Generate client keypair
      const keyPair = generateKeyPair();
      
      // Save client key
      const newClientKey: ClientKey = {
        id: clientId,
        publicKey: keyPair.publicKey,
        privateKey: keyPair.privateKey,
        createdAt: new Date().toISOString(),
      };
      setClientKeys(prev => [...prev, newClientKey]);

      // Send public key to server (base64 encoded for transport)
      const encodedPublicKey = btoa(keyPair.publicKey);
      
      const res = await fetch('/api/v1/register/complete', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          client_id: clientId,
          encrypted_client_public_key: {
            nonce: '',
            ciphertext: encodedPublicKey,
          },
        }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.message || 'Registration failed');
      
      setSession({
        session_id: clientId,
        api_key: data.api_key,
        expires_at: new Date(Date.now() + 3600000).toISOString(),
      });
      localStorage.setItem('omni_api_key', data.api_key);
      setPendingServerKey(null);
      setClientId('');
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    setSession(null);
    setIsAdmin(false);
    localStorage.removeItem('omni_api_key');
  };

  const handleAdminLogin = async () => {
    if (!adminKey.trim()) {
      setError('Please enter admin key');
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/admin/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ admin_key: adminKey }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error('Invalid admin key');
      setIsAdmin(true);
      setAdminKey('');
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const TabButton = ({ tab, label }: { tab: Tab; label: string }) => (
    <button
      onClick={() => setActiveTab(tab)}
      className={`flex-1 py-2 text-xs font-medium transition-colors ${
        activeTab === tab
          ? 'bg-blue-600 text-white'
          : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
      } ${tab === 'home' ? 'rounded-l-lg' : ''} ${tab === 'settings' ? 'rounded-r-lg' : ''}`}
    >
      {label}
    </button>
  );

  return (
    <main className="flex min-h-screen flex-col items-center p-4 pt-6">
      <div className="w-full max-w-md space-y-4">
        <div className="text-center mb-4">
          <h1 className="text-2xl font-bold mb-1">Omni Core</h1>
          <p className="text-slate-400 text-xs">Encrypted Key Exchange</p>
        </div>

        {/* Tab Navigation */}
        <div className="flex">
          <TabButton tab="home" label="Home" />
          <TabButton tab="register" label="Register" />
          <TabButton tab="client-keys" label="Keys" />
          <TabButton tab="server-keys" label="Server" />
          <TabButton tab="settings" label="Settings" />
        </div>

        {error && (
          <div className="bg-red-500/20 border border-red-500 rounded-lg p-3 text-red-300 text-sm">
            {error}
          </div>
        )}

        {/* Home Tab - Server Public Key QR + Admin Login */}
        {activeTab === 'home' && (
          <div className="bg-slate-800 rounded-lg p-5 space-y-4">
            {/* Server Public Key with QR */}
            <div className="text-center space-y-3">
              <h2 className="text-lg font-semibold">Server Public Key</h2>
              {serverInfo ? (
                <>
                  <div className="bg-white p-4 rounded-lg inline-block">
                    <QRCodeSVG 
                      value={serverInfo.server_public_key} 
                      size={160}
                      level="M"
                    />
                  </div>
                  <div className="space-y-2">
                    <p className="text-xs text-slate-400">Scan or copy to connect</p>
                    <div 
                      className="bg-slate-900 rounded p-2 font-mono text-xs break-all cursor-pointer hover:bg-slate-700"
                      onClick={() => copyToClipboard(serverInfo.server_public_key)}
                      title="Click to copy"
                    >
                      {serverInfo.server_public_key}
                    </div>
                    <p className="text-xs text-slate-500">
                      {serverInfo.server_name} v{serverInfo.version}
                    </p>
                  </div>
                </>
              ) : (
                <p className="text-slate-400 text-sm">Loading server info...</p>
              )}
            </div>

            <hr className="border-slate-700" />

            {/* Admin Login */}
            <div className="space-y-3">
              <h3 className="text-sm font-medium text-slate-300">Admin Login</h3>
              {isAdmin ? (
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full" />
                  <span className="text-green-400 text-sm">Logged in as Admin</span>
                  <button
                    onClick={handleLogout}
                    className="ml-auto text-xs text-red-400 hover:text-red-300"
                  >
                    Logout
                  </button>
                </div>
              ) : (
                <div className="flex gap-2">
                  <input
                    type="password"
                    value={adminKey}
                    onChange={(e) => setAdminKey(e.target.value)}
                    placeholder="Enter admin key"
                    className="flex-1 bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-blue-500"
                  />
                  <button
                    onClick={handleAdminLogin}
                    disabled={loading || !adminKey.trim()}
                    className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded px-4 py-2 text-sm font-medium"
                  >
                    Login
                  </button>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Register Tab */}
        {activeTab === 'register' && (
          <div className="bg-slate-800 rounded-lg p-5 space-y-4">
            {session ? (
              <>
                <div className="flex items-center gap-2 mb-4">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                  <span className="text-green-400 text-sm font-medium">Registered as {session.session_id}</span>
                </div>
                <div className="space-y-2">
                  <label className="text-xs text-slate-400">API Key</label>
                  <div className="bg-slate-900 rounded p-2 font-mono text-xs break-all">
                    {session.api_key}
                  </div>
                </div>
                <button
                  onClick={handleLogout}
                  className="w-full bg-red-600 hover:bg-red-700 rounded-lg py-2 text-sm font-medium"
                >
                  Logout
                </button>
              </>
            ) : pendingServerKey ? (
              <>
                <p className="text-slate-300 text-sm">
                  Server generated a keypair for <strong>{clientId}</strong>. 
                  Click below to generate your keypair and complete registration.
                </p>
                <div className="space-y-2">
                  <label className="text-xs text-slate-400">Server Public Key</label>
                  <div className="bg-slate-900 rounded p-2 font-mono text-xs break-all">
                    {pendingServerKey.slice(0, 32)}...
                  </div>
                </div>
                <button
                  onClick={handleRegisterComplete}
                  disabled={loading}
                  className="w-full bg-green-600 hover:bg-green-700 disabled:opacity-50 rounded-lg py-3 font-medium"
                >
                  {loading ? 'Generating...' : 'Generate Keys & Complete'}
                </button>
              </>
            ) : (
              <>
                <p className="text-slate-300 text-sm">
                  Enter a unique client ID to register with the server.
                </p>
                <input
                  type="text"
                  value={clientId}
                  onChange={(e) => setClientId(e.target.value)}
                  placeholder="my-device-001"
                  className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-blue-500"
                />
                <button
                  onClick={handleRegisterInit}
                  disabled={loading || !clientId.trim()}
                  className="w-full bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded-lg py-3 font-medium"
                >
                  {loading ? 'Registering...' : 'Start Registration'}
                </button>
              </>
            )}
          </div>
        )}

        {/* Client Keys Tab */}
        {activeTab === 'client-keys' && (
          <div className="bg-slate-800 rounded-lg p-5 space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-lg font-semibold">My Keypairs</h2>
              {clientKeys.length > 0 && (
                <button
                  onClick={() => setShowMyQR(!showMyQR)}
                  className="text-xs text-blue-400 hover:text-blue-300"
                >
                  {showMyQR ? 'Hide QR' : 'Show QR'}
                </button>
              )}
            </div>
            {clientKeys.length === 0 ? (
              <p className="text-slate-400 text-sm">No keys generated yet. Register to create a keypair.</p>
            ) : (
              <div className="space-y-3">
                {clientKeys.map((key) => (
                  <div key={key.id} className="bg-slate-900 rounded-lg p-3 space-y-2">
                    <div className="flex justify-between items-center">
                      <span className="font-medium text-sm">{key.id}</span>
                      <span className="text-xs text-slate-500">
                        {new Date(key.createdAt).toLocaleDateString()}
                      </span>
                    </div>
                    
                    {/* QR Code for sharing */}
                    {showMyQR && (
                      <div className="flex justify-center py-2">
                        <div className="bg-white p-3 rounded-lg">
                          <QRCodeSVG 
                            value={key.publicKey} 
                            size={120}
                            level="M"
                          />
                        </div>
                      </div>
                    )}
                    
                    <div className="space-y-1">
                      <label className="text-xs text-slate-400">Public Key</label>
                      <div 
                        className="font-mono text-xs text-green-400 break-all cursor-pointer hover:bg-slate-800 rounded p-1"
                        onClick={() => copyToClipboard(key.publicKey)}
                        title="Click to copy"
                      >
                        {showMyQR ? key.publicKey : `${key.publicKey.slice(0, 32)}...`}
                      </div>
                    </div>
                    <div className="space-y-1">
                      <label className="text-xs text-slate-400">Private Key (hidden)</label>
                      <div className="font-mono text-xs text-red-400">
                        ••••••••••••••••
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {/* Server Tab - Known Servers + Server Keys */}
        {activeTab === 'server-keys' && (
          <div className="bg-slate-800 rounded-lg p-5 space-y-4">
            {/* Known Omni Servers */}
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-semibold">Known Servers</h2>
                <button
                  onClick={fetchKnownServers}
                  className="text-xs text-blue-400 hover:text-blue-300"
                >
                  Refresh
                </button>
              </div>
              {knownServers.length === 0 ? (
                <p className="text-slate-400 text-sm">No other servers discovered yet.</p>
              ) : (
                <div className="space-y-2">
                  {knownServers.map((server) => (
                    <div key={server.server_id} className="bg-slate-900 rounded-lg p-3 space-y-1">
                      <div className="flex justify-between items-center">
                        <span className="font-medium text-sm">{server.name}</span>
                        <span className={`text-xs px-2 py-0.5 rounded ${server.is_authenticated ? 'bg-green-600' : 'bg-slate-600'}`}>
                          {server.is_authenticated ? 'Connected' : 'Known'}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400">{server.public_url}</p>
                      <div className="font-mono text-xs text-blue-400 truncate">
                        {server.public_key.slice(0, 24)}...
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            <hr className="border-slate-700" />

            {/* Per-Client Server Keys */}
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <h3 className="text-sm font-medium text-slate-300">Client Server Keys</h3>
                <button
                  onClick={fetchServerKeys}
                  className="text-xs text-blue-400 hover:text-blue-300"
                >
                  Refresh
                </button>
              </div>
              {serverKeys.length === 0 ? (
                <p className="text-slate-400 text-sm">No client keys yet.</p>
              ) : (
                <div className="space-y-2">
                  {serverKeys.map((key) => (
                    <div key={key.clientId} className="bg-slate-900 rounded p-2 space-y-1">
                      <span className="font-medium text-xs">{key.clientId}</span>
                      <div className="font-mono text-xs text-slate-500 truncate">
                        {key.publicKey.slice(0, 24)}...
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}

        {/* Settings Tab */}
        {activeTab === 'settings' && (
          <div className="bg-slate-800 rounded-lg p-5 space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-lg font-semibold">Server Settings</h2>
              {settingsSaved && (
                <span className="text-green-400 text-xs">✓ Saved</span>
              )}
            </div>

            {!isAdmin ? (
              <div className="text-center py-4">
                <p className="text-slate-400 text-sm mb-2">Admin login required to edit settings</p>
                <button
                  onClick={() => setActiveTab('home')}
                  className="text-blue-400 text-sm hover:text-blue-300"
                >
                  Go to Home to login
                </button>
              </div>
            ) : serverConfig ? (
              <div className="space-y-4">
                {/* Server Identity */}
                <div className="space-y-2">
                  <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">Server Identity</h3>
                  <div className="space-y-2">
                    <div>
                      <label className="text-xs text-slate-400">Server Name</label>
                      <input
                        type="text"
                        value={serverConfig.server.name}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          server: { ...serverConfig.server, name: e.target.value }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-slate-400">Description</label>
                      <input
                        type="text"
                        value={serverConfig.server.description}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          server: { ...serverConfig.server, description: e.target.value }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                  </div>
                </div>

                {/* Network */}
                <div className="space-y-2">
                  <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">Network</h3>
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <label className="text-xs text-slate-400">Host</label>
                      <input
                        type="text"
                        value={serverConfig.network.host}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          network: { ...serverConfig.network, host: e.target.value }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-slate-400">Port</label>
                      <input
                        type="number"
                        value={serverConfig.network.port}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          network: { ...serverConfig.network, port: parseInt(e.target.value) || 8080 }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                  </div>
                  <div>
                    <label className="text-xs text-slate-400">Public URL</label>
                    <input
                      type="text"
                      value={serverConfig.network.public_url}
                      onChange={(e) => setServerConfig({
                        ...serverConfig,
                        network: { ...serverConfig.network, public_url: e.target.value }
                      })}
                      placeholder="https://my-server.example.com"
                      className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white placeholder-slate-500"
                    />
                  </div>
                </div>

                {/* Auth */}
                <div className="space-y-2">
                  <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">Authentication</h3>
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <label className="text-xs text-slate-400">Session TTL (seconds)</label>
                      <input
                        type="number"
                        value={serverConfig.auth.session_ttl_secs}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          auth: { ...serverConfig.auth, session_ttl_secs: parseInt(e.target.value) || 3600 }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-slate-400">Admin Multiplier</label>
                      <input
                        type="number"
                        value={serverConfig.auth.admin_session_multiplier}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          auth: { ...serverConfig.auth, admin_session_multiplier: parseInt(e.target.value) || 24 }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                  </div>
                </div>

                {/* Federation */}
                <div className="space-y-2">
                  <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">Federation</h3>
                  <div className="flex gap-4">
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={serverConfig.federation.enabled}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          federation: { ...serverConfig.federation, enabled: e.target.checked }
                        })}
                        className="rounded bg-slate-900 border-slate-700"
                      />
                      <span className="text-slate-300">Enabled</span>
                    </label>
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={serverConfig.federation.public}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          federation: { ...serverConfig.federation, public: e.target.checked }
                        })}
                        className="rounded bg-slate-900 border-slate-700"
                      />
                      <span className="text-slate-300">Public</span>
                    </label>
                  </div>
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <label className="text-xs text-slate-400">Sync Interval (sec)</label>
                      <input
                        type="number"
                        value={serverConfig.federation.sync_interval_secs}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          federation: { ...serverConfig.federation, sync_interval_secs: parseInt(e.target.value) || 3600 }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-slate-400">Max Servers</label>
                      <input
                        type="number"
                        value={serverConfig.federation.max_known_servers}
                        onChange={(e) => setServerConfig({
                          ...serverConfig,
                          federation: { ...serverConfig.federation, max_known_servers: parseInt(e.target.value) || 1000 }
                        })}
                        className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
                      />
                    </div>
                  </div>
                </div>

                {/* Save Button */}
                <button
                  onClick={saveSettings}
                  disabled={loading}
                  className="w-full bg-green-600 hover:bg-green-700 disabled:opacity-50 rounded-lg py-3 font-medium"
                >
                  {loading ? 'Saving...' : 'Save Settings'}
                </button>
                <p className="text-xs text-slate-500 text-center">
                  Note: Network changes require server restart
                </p>
              </div>
            ) : (
              <p className="text-slate-400 text-sm">Loading settings...</p>
            )}
          </div>
        )}

        <div className="text-center text-xs text-slate-500 pt-4">
          <a href="/api/v1/health" className="hover:text-slate-300">Health</a>
          {' • '}
          <a href="/api/v1/servers/public" className="hover:text-slate-300">Servers</a>
          {' • '}
          <a href="/api/v1/servers/stats" className="hover:text-slate-300">Stats</a>
        </div>
      </div>
    </main>
  );
}
