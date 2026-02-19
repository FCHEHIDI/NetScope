# 01 — Méthodologie de conception

## 1) Objectif

Concevoir un MVP technique d’observabilité réseau pour un pipeline:

Kafka → service interne (normalisation + résilience) → API externe,

avec exposition temps réel et audit complet sur Ceph (RGW/S3).

## 2) Principes directeurs

1. **Ceph-first**: aucun contournement MinIO au MVP.
2. **Simplicité opérationnelle**: architecture modulaire minimale, sans composants non justifiés.
3. **Observabilité by design**: chaque requête est mesurable, corrélable, auditée.
4. **Résilience explicite**: timeouts, retries, backoff, circuit breaker et backpressure sont centralisés.
5. **Testabilité forte**: interfaces claires, dépendances injectables, scénarios de charge reproductibles.

## 3) Invariants réseau critiques

### I1 — Idempotence
- Toute requête sortante porte une clé d’idempotence stable (`idempotency_key`).
- Les retries ne doivent pas créer d’effet de bord métier.

### I2 — Timeouts cohérents
- Timeout connect, timeout lecture, timeout global sont distincts.
- Toute fin de timeout est observable (`timeout_type`, `elapsed_ms`).

### I3 — Cohérence métrique
- Horodatage monotone pour les mesures de latence.
- Buckets histogrammes stables entre environnements.

### I4 — Traçabilité bout en bout
- Propagation de `trace_id` et `span_id` sur tout le chemin.
- Aucun événement critique sans corrélation.

### I5 — Isolation interne/externe
- Distinction explicite IP interne et IP externe simulée (NAT).
- Les métriques et logs conservent les deux dimensions.

## 4) Approche méthodologique (phases)

1. **Cadrage**: invariants + contrats d’interface + SLO initiaux.
2. **Conception**: topologie réseau, flux nominaux et dégradés.
3. **Modélisation**: schémas des événements, audits, métriques, états résilience.
4. **Validation**: tests fonctionnels + charge + chaos léger réseau.
5. **Industrialisation**: hardening, tuning, trajectoire Kubernetes.

## 5) SLO/SLI initiaux (MVP)

- Latence sortante p95 < 400 ms (hors incidents externes majeurs).
- Taux d’erreur transport + HTTP >=500 < 2% en régime nominal.
- Perte d’événements d’audit = 0 (avec file locale de secours en cas RGW indisponible).
- Délai de visibilité dashboard temps réel < 2 s.

## 6) Livrables de cette phase

- Topologies et séquences réseau validées.
- Modèle de données commun (métriques/traces/audit).
- Plan de validation avec critères d’acceptation mesurables.
- Découpage implémentable en lots incrémentaux.
