# Build stage
FROM node:20-slim as builder

WORKDIR /app

# Copy package files
COPY frontend/package*.json ./

# Install dependencies
RUN npm install

# Copy source
COPY frontend/ ./

# Build
RUN npm run build

# Runtime stage
FROM node:20-slim

WORKDIR /app

# Copy built app
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

# Create public dir (may not exist)
RUN mkdir -p ./public

# Expose port
EXPOSE 3000

# Start
CMD ["npm", "start"]
