# syntax = docker/dockerfile:1

# Adjust NODE_VERSION as desired
ARG NODE_VERSION=20.17.0
FROM node:${NODE_VERSION}-slim AS base

LABEL fly_launch_runtime="SvelteKit"

# SvelteKit app lives here
WORKDIR /app

# Set production environment
ENV NODE_ENV="production"


# Throw-away build stage to reduce size of final image
FROM base AS build

# Install packages needed to build node modules
RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y build-essential node-gyp pkg-config python-is-python3

# Install node modules
COPY .npmrc package-lock.json package.json ./
RUN npm ci --include=dev

# Copy application code
COPY . .

# Vite inlines VITE_* variables at build time, so they must be supplied as
# build args (e.g. fly deploy --build-arg VITE_API_BASE_URL=https://api.example).
ARG VITE_API_BASE_URL
ARG VITE_MAPTILER_API_KEY
ENV VITE_API_BASE_URL=$VITE_API_BASE_URL
ENV VITE_MAPTILER_API_KEY=$VITE_MAPTILER_API_KEY

# Build application
RUN npm run build

# Remove development dependencies
RUN npm prune --omit=dev


# Final stage for app image - Use Nginx to serve static files
FROM nginx:1.25-alpine AS final

# Remove default nginx static assets
RUN rm -rf /usr/share/nginx/html/*

# SPA routing + caching rules (try_files fallback to /index.html)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy static assets from build stage to Nginx html directory
COPY --from=build /app/build /usr/share/nginx/html

# Ensure nginx user can read the files
RUN chown -R nginx:nginx /usr/share/nginx/html && chmod -R 755 /usr/share/nginx/html

# Expose Nginx default port
EXPOSE 80

# Nginx runs automatically when the container starts, no CMD needed explicitly here
# CMD ["nginx", "-g", "daemon off;"] # Default Nginx command
