'use client';

import { useState, useEffect } from 'react';
import type { Tab, Session, ServerKey } from '@/types';
import { TabButton } from '@/components/common';
import { HomeTab, RegisterTab, KeysTab, ServerTab, SettingsTab } from '@/components/tabs';
import { useServerInfo, useSettings, useAuth, useKnownServers, useClientKeys } from '@/hooks';
import { api } from '@/lib/api';

export default function Home() {
  const [activeTab, setActiveTab] = useState<Tab>('home');
  const [session, setSession] = useState<Session | null>(null);
  const [serverKeys, setServerKeys] = useState<ServerKey[]>([]);
  const [error, setError] = useState<string | null>(null);

  // Custom hooks for data fetching
  const { serverInfo } = useServerInfo();
  const { config, setConfig, saving, saved, save } = useSettings();
  const { isAdmin, loading: authLoading, login, logout } = useAuth();
  const { servers: knownServers, refetch: refetchServers } = useKnownServers();
  const { keys: clientKeys, addKey: addClientKey } = useClientKeys();

  // Fetch server keys on mount
  useEffect(() => {
    fetchServerKeys();
  }, []);

  const fetchServerKeys = async () => {
    try {
      const data = await api.getServerKeys();
      setServerKeys(
        data.keys.map((k) => ({
          clientId: k.client_id,
          publicKey: k.public_key,
        }))
      );
    } catch {
      // Ignore errors on initial load
    }
  };

  const handleLogout = () => {
    setSession(null);
    logout();
  };

  const handleSaveSettings = async () => {
    if (config) {
      await save(config);
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center p-4 pt-6">
      <div className="w-full max-w-md space-y-4">
        <div className="text-center mb-4">
          <h1 className="text-2xl font-bold mb-1">Omni Core</h1>
          <p className="text-slate-400 text-xs">Encrypted Key Exchange</p>
        </div>

        {/* Tab Navigation */}
        <div className="flex">
          <TabButton tab="home" label="Home" activeTab={activeTab} onClick={setActiveTab} />
          <TabButton tab="register" label="Register" activeTab={activeTab} onClick={setActiveTab} />
          <TabButton tab="client-keys" label="Keys" activeTab={activeTab} onClick={setActiveTab} />
          <TabButton tab="server-keys" label="Server" activeTab={activeTab} onClick={setActiveTab} />
          <TabButton tab="settings" label="Settings" activeTab={activeTab} onClick={setActiveTab} />
        </div>

        {error && (
          <div className="bg-red-500/20 border border-red-500 rounded-lg p-3 text-red-300 text-sm">
            {error}
          </div>
        )}

        {/* Tab Content */}
        {activeTab === 'home' && (
          <HomeTab
            serverInfo={serverInfo}
            isAdmin={isAdmin}
            onAdminLogin={login}
            onLogout={handleLogout}
            loading={authLoading}
          />
        )}

        {activeTab === 'register' && (
          <RegisterTab
            session={session}
            setSession={setSession}
            onAddClientKey={addClientKey}
            onLogout={handleLogout}
          />
        )}

        {activeTab === 'client-keys' && <KeysTab keys={clientKeys} />}

        {activeTab === 'server-keys' && (
          <ServerTab
            knownServers={knownServers}
            serverKeys={serverKeys}
            onRefreshServers={refetchServers}
            onRefreshKeys={fetchServerKeys}
          />
        )}

        {activeTab === 'settings' && (
          <SettingsTab
            isAdmin={isAdmin}
            config={config}
            setConfig={setConfig}
            onSave={handleSaveSettings}
            saving={saving}
            saved={saved}
            onGoToLogin={() => setActiveTab('home')}
          />
        )}

        <div className="text-center text-xs text-slate-500 pt-4">
          <a href="/api/v1/health" className="hover:text-slate-300">
            Health
          </a>
          {' • '}
          <a href="/api/v1/servers/public" className="hover:text-slate-300">
            Servers
          </a>
          {' • '}
          <a href="/api/v1/servers/stats" className="hover:text-slate-300">
            Stats
          </a>
        </div>
      </div>
    </main>
  );
}
