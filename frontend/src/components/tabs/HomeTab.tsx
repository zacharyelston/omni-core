'use client';

import { useState } from 'react';
import { QRCodeSVG } from 'qrcode.react';
import type { ServerInfo } from '@/types';

type HomeTabProps = {
  serverInfo: ServerInfo | null;
  isAdmin: boolean;
  onAdminLogin: (key: string) => Promise<boolean>;
  onLogout: () => void;
  loading: boolean;
};

export function HomeTab({
  serverInfo,
  isAdmin,
  onAdminLogin,
  onLogout,
  loading,
}: HomeTabProps) {
  const [adminKey, setAdminKey] = useState('');

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleLogin = async () => {
    const success = await onAdminLogin(adminKey);
    if (success) {
      setAdminKey('');
    }
  };

  return (
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
              onClick={onLogout}
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
              onClick={handleLogin}
              disabled={loading || !adminKey.trim()}
              className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded px-4 py-2 text-sm font-medium"
            >
              Login
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
