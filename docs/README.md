# NetScope — Dossier méthodologique

Ce dossier formalise la méthode de conception du MVP d’observabilité réseau (Ceph-first) pour un pipeline distribué inspiré télécom/SRE.

## Contenu

1. [01-methodologie.md](01-methodologie.md) — cadre méthodologique, périmètre, invariants, principes d’architecture
2. [02-topologie-flux.md](02-topologie-flux.md) — topologies cibles, flux réseau, séquences, gestion d’erreur
3. [03-modelisation-donnees.md](03-modelisation-donnees.md) — modélisation relationnelle/logique des métriques, traces, audits
4. [04-plan-validation.md](04-plan-validation.md) — stratégie de validation, charge, critères d’acceptation, gouvernance

## Positionnement

- Priorité MVP: robustesse fonctionnelle, traçabilité, mesure réseau fiable
- Stockage objet: Ceph RGW dès la V1
- Évolution prévue: industrialisation Kubernetes et montée en charge multi-VM

## Résultat attendu

À l’issue de cette phase documentaire, l’équipe dispose d’un blueprint exécutable qui limite l’over-engineering et sécurise l’implémentation incrémentale.
