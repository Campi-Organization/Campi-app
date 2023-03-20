FROM rust:1.68
WORKDIR /api
ENV DATABASE_URL = "mysql://user:password@db:3306/CAMPI"
COPY . .
RUN cargo prisma generate
COPY . .
EXPOSE 3000
CMD ["cargo", "dev"]

