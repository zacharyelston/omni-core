'use client';

import { useState } from 'react';
import type { Session, ClientKey } from '@/types';
import { api } from '@/lib/api';

type RegisterTabProps = {
  session: Session | null;
  setSession: (session: Session | null) => void;
  onAddClientKey: (key: ClientKey) => void;
  onLogout: () => void;
};

export function RegisterTab({
  session,
  setSession,
  onAddClientKey,
  onLogout,
}: RegisterTabProps) {
  const [clientId, setClientId] = useState('');
  const [pendingServerKey, setPendingServerKey] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const generateKeyPair = (): { publicKey: string; privateKey: string } => {
    const bytes = new Uint8Array(32);
    crypto.getRandomValues(bytes);
    const privateKey = Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
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
      const data = await api.registerInit(clientId);
      setPendingServerKey(data.server_public_key);
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
      const keyPair = generateKeyPair();
      const newClientKey: ClientKey = {
        id: clientId,
        publicKey: keyPair.publicKey,
        privateKey: keyPair.privateKey,
        createdAt: new Date().toISOString(),
      };
      onAddClientKey(newClientKey);

      const encodedPublicKey = btoa(keyPair.publicKey);
      const data = await api.registerComplete(clientId, {
        nonce: '',
        ciphertext: encodedPublicKey,
      });

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

  return (
    <div className="bg-slate-800 rounded-lg p-5 space-y-4">
      {error && (
        <div className="bg-red-500/20 border border-red-500 rounded-lg p-3 text-red-300 text-sm">
          {error}
        </div>
      )}

      {session ? (
        <>
          <div className="flex items-center gap-2 mb-4">
            <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
            <span className="text-green-400 text-sm font-medium">
              Registered as {session.session_id}
            </span>
          </div>
          <div className="space-y-2">
            <label className="text-xs text-slate-400">API Key</label>
            <div className="bg-slate-900 rounded p-2 font-mono text-xs break-all">
              {session.api_key}
            </div>
          </div>
          <button
            onClick={onLogout}
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
  );
}
