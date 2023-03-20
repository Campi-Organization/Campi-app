FROM rust:1.68-alpine3.17
WORKDIR /api
ENV DATABASE_URL = "mysql://user:password@db:3306/CAMPI"
COPY . .
RUN cargo prisma generate

COPY . .
CMD ["cargo", "dev"]

