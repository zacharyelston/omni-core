'use client';

import { useState } from 'react';
import { QRCodeSVG } from 'qrcode.react';
import type { ClientKey } from '@/types';

type KeysTabProps = {
  keys: ClientKey[];
};

export function KeysTab({ keys }: KeysTabProps) {
  const [showQR, setShowQR] = useState(false);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="bg-slate-800 rounded-lg p-5 space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-lg font-semibold">My Keypairs</h2>
        {keys.length > 0 && (
          <button
            onClick={() => setShowQR(!showQR)}
            className="text-xs text-blue-400 hover:text-blue-300"
          >
            {showQR ? 'Hide QR' : 'Show QR'}
          </button>
        )}
      </div>

      {keys.length === 0 ? (
        <p className="text-slate-400 text-sm">
          No keys generated yet. Register to create a keypair.
        </p>
      ) : (
        <div className="space-y-3">
          {keys.map((key) => (
            <div key={key.id} className="bg-slate-900 rounded-lg p-3 space-y-2">
              <div className="flex justify-between items-center">
                <span className="font-medium text-sm">{key.id}</span>
                <span className="text-xs text-slate-500">
                  {new Date(key.createdAt).toLocaleDateString()}
                </span>
              </div>

              {showQR && (
                <div className="flex justify-center py-2">
                  <div className="bg-white p-3 rounded-lg">
                    <QRCodeSVG value={key.publicKey} size={120} level="M" />
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
                  {showQR ? key.publicKey : `${key.publicKey.slice(0, 32)}...`}
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
  );
}
