'use client';

import type { KnownServer, ServerKey } from '@/types';

type ServerTabProps = {
  knownServers: KnownServer[];
  serverKeys: ServerKey[];
  onRefreshServers: () => void;
  onRefreshKeys: () => void;
};

export function ServerTab({
  knownServers,
  serverKeys,
  onRefreshServers,
  onRefreshKeys,
}: ServerTabProps) {
  return (
    <div className="bg-slate-800 rounded-lg p-5 space-y-4">
      {/* Known Omni Servers */}
      <div className="space-y-3">
        <div className="flex justify-between items-center">
          <h2 className="text-lg font-semibold">Known Servers</h2>
          <button
            onClick={onRefreshServers}
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
                  <span
                    className={`text-xs px-2 py-0.5 rounded ${
                      server.is_authenticated ? 'bg-green-600' : 'bg-slate-600'
                    }`}
                  >
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
            onClick={onRefreshKeys}
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
  );
}
