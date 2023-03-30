FROM rust:1.68
WORKDIR /api
ENV DATABASE_URL = "mysql://user:password@db:3306/CAMPI"
COPY . .
EXPOSE 3000
RUN cargo prisma generate
CMD ["cargo", "dev"]

