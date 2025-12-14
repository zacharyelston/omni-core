'use client';

import { useState } from 'react';

interface Session {
  session_id: string;
  api_key: string;
  expires_at: string;
}

export default function Home() {
  const [session, setSession] = useState<Session | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleJoin = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/auth/join', { method: 'POST' });
      if (!res.ok) throw new Error('Failed to join');
      const data = await res.json();
      setSession(data);
      localStorage.setItem('omni_api_key', data.api_key);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleVerify = async () => {
    const apiKey = session?.api_key || localStorage.getItem('omni_api_key');
    if (!apiKey) {
      setError('No API key found');
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/v1/auth/verify', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ api_key: apiKey }),
      });
      const data = await res.json();
      if (data.valid) {
        setSession({
          session_id: data.session_id,
          api_key: apiKey,
          expires_at: data.expires_at,
        });
      } else {
        setSession(null);
        localStorage.removeItem('omni_api_key');
        setError('Session expired or invalid');
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = async () => {
    const apiKey = session?.api_key || localStorage.getItem('omni_api_key');
    if (!apiKey) return;
    setLoading(true);
    try {
      await fetch('/api/v1/auth/logout', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ api_key: apiKey }),
      });
      setSession(null);
      localStorage.removeItem('omni_api_key');
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-6">
      <div className="w-full max-w-md space-y-6">
        <div className="text-center">
          <h1 className="text-4xl font-bold mb-2">Omni Core</h1>
          <p className="text-slate-400">Authentication & Session Management</p>
        </div>

        {error && (
          <div className="bg-red-500/20 border border-red-500 rounded-lg p-4 text-red-300">
            {error}
          </div>
        )}

        {session ? (
          <div className="bg-slate-800 rounded-lg p-6 space-y-4">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-green-500 rounded-full animate-pulse" />
              <span className="text-green-400 font-medium">Connected</span>
            </div>
            
            <div className="space-y-2">
              <label className="text-sm text-slate-400">Session ID</label>
              <div className="bg-slate-900 rounded p-3 font-mono text-sm break-all">
                {session.session_id}
              </div>
            </div>

            <div className="space-y-2">
              <label className="text-sm text-slate-400">API Key</label>
              <div className="bg-slate-900 rounded p-3 font-mono text-xs break-all">
                {session.api_key}
              </div>
            </div>

            <div className="space-y-2">
              <label className="text-sm text-slate-400">Expires</label>
              <div className="bg-slate-900 rounded p-3 text-sm">
                {new Date(session.expires_at).toLocaleString()}
              </div>
            </div>

            <div className="flex gap-3 pt-4">
              <button
                onClick={handleVerify}
                disabled={loading}
                className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded-lg py-3 font-medium transition-colors"
              >
                Verify
              </button>
              <button
                onClick={handleLogout}
                disabled={loading}
                className="flex-1 bg-red-600 hover:bg-red-700 disabled:opacity-50 rounded-lg py-3 font-medium transition-colors"
              >
                Logout
              </button>
            </div>
          </div>
        ) : (
          <div className="bg-slate-800 rounded-lg p-6 space-y-4">
            <p className="text-slate-300 text-center">
              Join to create a new session and receive an API key.
            </p>
            <button
              onClick={handleJoin}
              disabled={loading}
              className="w-full bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded-lg py-4 font-medium text-lg transition-colors"
            >
              {loading ? 'Connecting...' : 'Join'}
            </button>
            
            {localStorage.getItem('omni_api_key') && (
              <button
                onClick={handleVerify}
                disabled={loading}
                className="w-full bg-slate-700 hover:bg-slate-600 disabled:opacity-50 rounded-lg py-3 font-medium transition-colors"
              >
                Restore Session
              </button>
            )}
          </div>
        )}

        <div className="text-center text-sm text-slate-500">
          <a href="/api/v1/health" className="hover:text-slate-300">
            API Health Check
          </a>
        </div>
      </div>
    </main>
  );
}
