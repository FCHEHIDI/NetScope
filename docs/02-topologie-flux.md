# 02 — Topologies et flux réseau

## 1) Topologie logique MVP

```mermaid
graph LR
    A[Kafka Topic Ingress] --> B[Pipeline Worker Rust]
    B --> C[Resilience Layer\nTimeout Retry Backoff CB BP]
    C --> D[External API]

    B --> E[Network Probe]
    E --> F[Metrics Engine]
    F --> G[Prometheus]
    G --> H[Grafana]

    B --> I[Audit Logger]
    I --> J[Ceph RGW\nObject Storage]

    F --> K[REST API Axum]
    F --> L[WebSocket Gateway]
    L --> M[Realtime Dashboard]

    B -.trace_id/span_id.-> I
    B -.events.-> L
```

## 2) Topologie de déploiement (MVP Docker-first)

```mermaid
graph TD
    subgraph HOST[Host / VM de test]
        subgraph DOCKER[Docker Compose]
            SVC[service Rust\nworker+api+ws]
            KAFKA[Kafka + broker]
            CEPH[Ceph RGW + MON/OSD light]
            PROM[Prometheus]
            GRAF[Grafana]
            LOAD[Load Generator]
        end
    end

    LOAD --> SVC
    SVC --> KAFKA
    SVC --> CEPH
    SVC --> PROM
    GRAF --> PROM
```

## 3) Flux nominal (ingestion → externe → audit)

```mermaid
sequenceDiagram
    autonumber
    participant K as Kafka
    participant W as Worker
    participant R as Resilience
    participant X as API Externe
    participant P as Probe/Metrics
    participant A as Audit Logger
    participant C as Ceph RGW
    participant D as WS Dashboard

    K->>W: Message brut
    W->>W: Normalisation + validation
    W->>R: Requête sortante (trace_id)
    R->>X: HTTP request
    X-->>R: HTTP response
    R-->>W: résultat + metadata résilience
    W->>P: latence/status/retries/timeouts
    W->>A: audit event JSON
    A->>C: put_object (partitionné)
    W-->>D: event temps réel via WS
```

## 4) Flux dégradé (erreur + backpressure)

```mermaid
sequenceDiagram
    autonumber
    participant W as Worker
    participant R as Resilience
    participant X as API Externe
    participant P as Probe/Metrics
    participant A as Audit Logger

    W->>R: Appel sortant
    R->>X: tentative #1
    X-->>R: timeout
    R->>X: tentative #2 (backoff)
    X-->>R: 503
    R-->>W: échec final (circuit state=open?)
    W->>P: incrément erreurs/retries/timeouts
    W->>A: audit event ERROR
    W->>W: backpressure (queue bornée / throttle)
```

## 5) Règles de topologie

- Le probe réseau est au plus près du client HTTP sortant.
- Les politiques de résilience sont centralisées, pas dispersées dans les handlers.
- L’audit est asynchrone avec garantie de livraison (buffer local borné + reprise).
- Le dashboard WS est alimenté par événements déjà corrélés (trace_id obligatoire).
