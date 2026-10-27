# Stage 1: Build
FROM golang:1.25-alpine AS builder

RUN apk add --no-cache git ca-certificates tzdata

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

ENV CGO_ENABLED=0 GOOS=linux GOARCH=amd64
RUN go build -ldflags="-s -w" -o region .

# Stage 2: Runtime
FROM alpine:3.21

RUN apk add --no-cache ca-certificates tzdata curl \
    && addgroup -S nonroot && adduser -S nonroot -G nonroot

WORKDIR /app

COPY --from=builder /app/region ./region
COPY --from=builder /app/seed.json ./seed.json
COPY --from=builder /app/docs ./docs

RUN chown -R nonroot:nonroot /app

USER nonroot:nonroot

ENV PORT=5008
EXPOSE 5008

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
  CMD curl -f http://localhost:${PORT}/health || exit 1

ENTRYPOINT ["./region"]
