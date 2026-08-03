FROM node:24.16.0-alpine3.23@sha256:2bdb65ed1dab192432bc31c95f94155ca5ad7fc1392fb7eb7526ab682fa5bf14 AS build

WORKDIR /workspace
COPY . .
RUN npm ci --ignore-scripts
RUN npm exec -- vite apps/beaver-nest-fe build

FROM node:24.16.0-alpine3.23@sha256:2bdb65ed1dab192432bc31c95f94155ca5ad7fc1392fb7eb7526ab682fa5bf14

WORKDIR /app
COPY --from=build /workspace/apps/beaver-nest-fe/dist ./
COPY --from=build /workspace/node_modules ./node_modules

EXPOSE 19310
CMD ["./node_modules/.bin/vite", "preview", "--host", "0.0.0.0", "--port", "19310"]
