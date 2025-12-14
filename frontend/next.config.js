/** @type {import('next').NextConfig} */
const BACKEND_URL = process.env.BACKEND_URL || 'http://localhost:8080';

const nextConfig = {
  reactStrictMode: true,
  allowedDevOrigins: ['*.replit.dev', '*.replit.app'],
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: `${BACKEND_URL}/api/:path*`,
      },
    ];
  },
};

module.exports = nextConfig;
