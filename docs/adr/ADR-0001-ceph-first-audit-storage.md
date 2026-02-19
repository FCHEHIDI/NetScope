# ADR-0001 — Ceph-first pour le stockage d’audit

- **Statut**: accepted
- **Date**: 2026-02-20
- **Décideurs**: NetScope Core Team

## Contexte

Le MVP doit introduire Ceph en priorité, avec audit réseau persistant, traçable et reconstituable.
Le pipeline produit des événements critiques (requêtes, erreurs, retries, timeouts, états de résilience) qui ne doivent pas être perdus.

## Décision

Adopter **Ceph RGW (API S3-compatible)** comme stockage d’audit dès la V1.
Le format d’écriture est `JSONL` partitionné temporellement avec clés objets déterministes.

## Alternatives considérées

1. MinIO pour le MVP
   - Avantage: setup plus simple.
   - Rejeté: ne répond pas à l’objectif d’apprentissage Ceph-first.
2. Base relationnelle pour audit
   - Avantage: requêtage SQL direct.
   - Rejeté: coût d’exploitation supérieur et modèle moins adapté au flux append-only.

## Conséquences

### Positives
- Alignement direct avec objectif Ceph.
- Stockage audit immutable et scalable.
- Compatibilité opérationnelle avec workflows S3.

### Négatives
- Setup local plus complexe qu’un mock storage.
- Besoin de mécanisme de reprise en cas d’indisponibilité RGW.

## Mesures de contrôle

- Buffer local borné et durable en cas d’échec RGW.
- Retry progressif avec backoff.
- Alerte sur backlog audit et erreurs d’écriture.

## Impacts

- Modules impactés: `audit-ceph`, `domain`, `config`, `runbook`.
- Invariants impactés: I4 (traçabilité), I5 (cohérence métadonnées réseau).
