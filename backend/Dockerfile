# build stage
FROM node:lts-alpine as build-stage
WORKDIR /app
COPY package*.json ./

RUN npm install typescript -g

# RUN npm install
# If you are building your code for production
RUN npm ci

RUN npx prisma generate

# Bundle app source
COPY . .

# Set environment variables
ENV NODE_ENV production
RUN npm run build

FROM node:lts-alpine as production-stage
WORKDIR /app

COPY tsconfig.json ./
COPY package*.json ./
RUN npm ci

COPY package*.json ./
COPY --from=build-stage /app/build /app
COPY --from=build-stage /app/src/generated/ /app/src/generated/
COPY --from=build-stage /app/prisma/ /app/prisma/

EXPOSE 3000
CMD [ "node", "-r", "tsconfig-paths/register", "-r", "ts-node/register", "/app/src/index.js" ]