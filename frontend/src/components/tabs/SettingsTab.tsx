'use client';

import type { ServerConfig } from '@/types';

type SettingsTabProps = {
  isAdmin: boolean;
  config: ServerConfig | null;
  setConfig: (config: ServerConfig) => void;
  onSave: () => void;
  saving: boolean;
  saved: boolean;
  onGoToLogin: () => void;
};

export function SettingsTab({
  isAdmin,
  config,
  setConfig,
  onSave,
  saving,
  saved,
  onGoToLogin,
}: SettingsTabProps) {
  if (!isAdmin) {
    return (
      <div className="bg-slate-800 rounded-lg p-5 space-y-4">
        <h2 className="text-lg font-semibold">Server Settings</h2>
        <div className="text-center py-4">
          <p className="text-slate-400 text-sm mb-2">
            Admin login required to edit settings
          </p>
          <button
            onClick={onGoToLogin}
            className="text-blue-400 text-sm hover:text-blue-300"
          >
            Go to Home to login
          </button>
        </div>
      </div>
    );
  }

  if (!config) {
    return (
      <div className="bg-slate-800 rounded-lg p-5">
        <p className="text-slate-400 text-sm">Loading settings...</p>
      </div>
    );
  }

  return (
    <div className="bg-slate-800 rounded-lg p-5 space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-lg font-semibold">Server Settings</h2>
        {saved && <span className="text-green-400 text-xs">✓ Saved</span>}
      </div>

      {/* Server Identity */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">
          Server Identity
        </h3>
        <div className="space-y-2">
          <div>
            <label className="text-xs text-slate-400">Server Name</label>
            <input
              type="text"
              value={config.server.name}
              onChange={(e) =>
                setConfig({
                  ...config,
                  server: { ...config.server, name: e.target.value },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
          <div>
            <label className="text-xs text-slate-400">Description</label>
            <input
              type="text"
              value={config.server.description}
              onChange={(e) =>
                setConfig({
                  ...config,
                  server: { ...config.server, description: e.target.value },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
        </div>
      </div>

      {/* Network */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">
          Network
        </h3>
        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="text-xs text-slate-400">Host</label>
            <input
              type="text"
              value={config.network.host}
              onChange={(e) =>
                setConfig({
                  ...config,
                  network: { ...config.network, host: e.target.value },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
          <div>
            <label className="text-xs text-slate-400">Port</label>
            <input
              type="number"
              value={config.network.port}
              onChange={(e) =>
                setConfig({
                  ...config,
                  network: {
                    ...config.network,
                    port: parseInt(e.target.value) || 8080,
                  },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
        </div>
        <div>
          <label className="text-xs text-slate-400">Public URL</label>
          <input
            type="text"
            value={config.network.public_url}
            onChange={(e) =>
              setConfig({
                ...config,
                network: { ...config.network, public_url: e.target.value },
              })
            }
            placeholder="https://my-server.example.com"
            className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white placeholder-slate-500"
          />
        </div>
      </div>

      {/* Auth */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">
          Authentication
        </h3>
        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="text-xs text-slate-400">Session TTL (seconds)</label>
            <input
              type="number"
              value={config.auth.session_ttl_secs}
              onChange={(e) =>
                setConfig({
                  ...config,
                  auth: {
                    ...config.auth,
                    session_ttl_secs: parseInt(e.target.value) || 3600,
                  },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
          <div>
            <label className="text-xs text-slate-400">Admin Multiplier</label>
            <input
              type="number"
              value={config.auth.admin_session_multiplier}
              onChange={(e) =>
                setConfig({
                  ...config,
                  auth: {
                    ...config.auth,
                    admin_session_multiplier: parseInt(e.target.value) || 24,
                  },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
        </div>
      </div>

      {/* Federation */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-slate-300 border-b border-slate-700 pb-1">
          Federation
        </h3>
        <div className="flex gap-4">
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={config.federation.enabled}
              onChange={(e) =>
                setConfig({
                  ...config,
                  federation: { ...config.federation, enabled: e.target.checked },
                })
              }
              className="rounded bg-slate-900 border-slate-700"
            />
            <span className="text-slate-300">Enabled</span>
          </label>
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={config.federation.public}
              onChange={(e) =>
                setConfig({
                  ...config,
                  federation: { ...config.federation, public: e.target.checked },
                })
              }
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
              value={config.federation.sync_interval_secs}
              onChange={(e) =>
                setConfig({
                  ...config,
                  federation: {
                    ...config.federation,
                    sync_interval_secs: parseInt(e.target.value) || 3600,
                  },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
          <div>
            <label className="text-xs text-slate-400">Max Servers</label>
            <input
              type="number"
              value={config.federation.max_known_servers}
              onChange={(e) =>
                setConfig({
                  ...config,
                  federation: {
                    ...config.federation,
                    max_known_servers: parseInt(e.target.value) || 1000,
                  },
                })
              }
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white"
            />
          </div>
        </div>
      </div>

      {/* Save Button */}
      <button
        onClick={onSave}
        disabled={saving}
        className="w-full bg-green-600 hover:bg-green-700 disabled:opacity-50 rounded-lg py-3 font-medium"
      >
        {saving ? 'Saving...' : 'Save Settings'}
      </button>
      <p className="text-xs text-slate-500 text-center">
        Note: Network changes require server restart
      </p>
    </div>
  );
}
