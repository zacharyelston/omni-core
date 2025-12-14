# Frontend Guide

## Technology Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State**: React useState + localStorage

## Project Structure

```
frontend/
├── package.json
├── next.config.js      # API proxy config
├── tailwind.config.js
├── tsconfig.json
└── src/
    └── app/
        ├── layout.tsx  # Root layout, metadata
        ├── page.tsx    # Main tabbed UI
        └── globals.css # Tailwind imports
```

## Running

```bash
cd frontend
npm install
npm run dev
# http://localhost:5000
```

## Configuration

### next.config.js

API requests are proxied to the backend:

```javascript
async rewrites() {
  return [
    {
      source: '/api/:path*',
      destination: `${BACKEND_URL}/api/:path*`,
    },
  ];
}
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `BACKEND_URL` | http://localhost:8080 | Backend API URL |

## Components

### Tab Navigation

Three tabs for different views:
- **Register** - Two-step registration flow
- **My Keys** - Client's own keypairs
- **Server Keys** - Known server public keys

### State Management

```typescript
const [activeTab, setActiveTab] = useState<Tab>('register');
const [session, setSession] = useState<Session | null>(null);
const [clientKeys, setClientKeys] = useState<ClientKey[]>([]);
const [serverKeys, setServerKeys] = useState<ServerKey[]>([]);
```

### Local Storage

Client keys are persisted:
```typescript
// Load on mount
useEffect(() => {
  const saved = localStorage.getItem('omni_client_keys');
  if (saved) setClientKeys(JSON.parse(saved));
}, []);

// Save on change
useEffect(() => {
  localStorage.setItem('omni_client_keys', JSON.stringify(clientKeys));
}, [clientKeys]);
```

## Registration Flow

1. User enters client ID
2. Click "Start Registration"
3. Server returns its public key
4. Click "Generate Keys & Complete"
5. Client generates keypair
6. Sends public key to server
7. Receives API key

## Mobile-First Design

- Responsive max-width container
- Touch-friendly button sizes (py-3)
- Viewport meta for mobile scaling
- Dark theme for OLED screens

## Adding Features

1. Add state in `page.tsx`
2. Create UI component
3. Add API call function
4. Handle loading/error states

Example API call:
```typescript
const handleAction = async () => {
  setLoading(true);
  setError(null);
  try {
    const res = await fetch('/api/v1/endpoint', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ data }),
    });
    if (!res.ok) throw new Error('Failed');
    const data = await res.json();
    // Update state
  } catch (e) {
    setError(e.message);
  } finally {
    setLoading(false);
  }
};
```

## Building for Production

```bash
npm run build
npm start
```
