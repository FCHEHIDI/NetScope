# 06 — Runbook opérationnel MVP (Ceph-first)

## 1) Objet et périmètre

Ce runbook décrit les opérations standard du MVP NetScope:
- ingestion Kafka
- pipeline Rust (worker + API REST + WebSocket)
- audit sur Ceph RGW
- observabilité via Prometheus/Grafana

Objectif: garantir une exploitation reproductible, diagnostiquer rapidement les incidents et préserver les invariants réseau (I1→I5).

## 2) Pré-requis d’exploitation

- Environnement Docker Compose fonctionnel.
- Accès réseau aux composants: Kafka, service Rust, Ceph RGW, Prometheus, Grafana.
- Credentials Ceph RGW dédiés au projet (lecture/écriture bucket audit).
- NTP/synchronisation horaire active sur l’hôte.

## 3) Démarrage standard (SOP)

## Étapes

1. Démarrer l’infra de base (Kafka, Ceph RGW, Prometheus, Grafana).
2. Vérifier la disponibilité des dépendances (ports + endpoints santé).
3. Démarrer le service NetScope (worker/API/WS).
4. Vérifier l’ingestion, la publication métriques et l’écriture audit.
5. Activer la génération de trafic de test nominal.

## Contrôles de succès

- `GET /health/live` retourne `200`.
- `GET /health/ready` retourne `200` uniquement si Kafka + RGW + engine métriques sont prêts.
- Les compteurs `requests_total` et `audit_writes_total` augmentent.
- Le dashboard WebSocket reçoit des événements corrélés (`trace_id` présent).

## 4) Arrêt contrôlé

## Étapes

1. Arrêter la génération de charge.
2. Drainer les files internes (timeout borné).
3. Fermer proprement consumer Kafka.
4. Flusher les buffers d’audit vers Ceph.
5. Arrêter les services applicatifs puis l’infra.

## Critères d’arrêt correct

- Aucun message non traité restant dans la file applicative.
- Aucun lot d’audit en attente d’écriture.
- Aucun processus bloqué en arrêt forcé.

## 5) Checklist de santé (toutes les 5 minutes)

- **Service**: `live=UP`, `ready=UP`.
- **Kafka**: consumer lag stable, pas de dérive continue.
- **Ceph RGW**: latence d’écriture stable, taux d’erreur nul/faible.
- **Métriques**:
  - p95/p99 latence dans la plage attendue,
  - `errors_total` et `timeouts_total` sous seuil,
  - `queue_depth` non croissant,
  - `circuit_state` majoritairement `closed`.
- **WS**: délai de diffusion < 2 s en charge nominale.

## 6) Matrice d’incidents et réponses

## INC-01 — Hausse brutale des timeouts sortants

### Symptômes
- montée de `timeouts_total`
- hausse de p95/p99
- retries en augmentation

### Actions
1. Vérifier la latence/rate-limit de l’API externe.
2. Contrôler les paramètres timeout (connect/read/global).
3. Vérifier l’ouverture du circuit breaker.
4. Réduire temporairement le débit entrant (throttle/backpressure).

### Validation retour à la normale
- p95/p99 redescendent
- `timeouts_total` revient au niveau de base
- circuit revient `closed`

## INC-02 — Erreurs d’écriture Ceph RGW

### Symptômes
- augmentation `audit_write_errors_total`
- backlog buffer audit en hausse

### Actions
1. Vérifier access keys/secret et permissions bucket.
2. Vérifier disponibilité endpoint RGW et latence réseau.
3. Activer mode dégradé: buffer local durable + retry progressif.
4. Surveiller saturation disque locale (zone buffer).

### Validation retour à la normale
- reprise des écritures RGW
- purge progressive du backlog audit
- aucune perte d’événement critique

## INC-03 — Circuit breaker ouvert durablement

### Symptômes
- `circuit_state=open` persistant
- chute du débit utile

### Actions
1. Identifier cause racine (upstream indisponible, latence extrême, erreurs 5xx).
2. Vérifier paramètres seuils du breaker.
3. Activer mode dégradé contrôlé (rejet explicite / file limitée).
4. Revenir progressivement en half-open selon politique définie.

### Validation retour à la normale
- transitions open → half-open → closed conformes
- stabilité des erreurs sous seuil

## 7) Procédure de replay et reconstitution

## Replay Kafka

1. Identifier l’intervalle incident (timestamps).
2. Réinitialiser offset de consumer sur fenêtre ciblée.
3. Rejouer en débit contrôlé.
4. Vérifier idempotence (`idempotency_key`) pour éviter duplications.

## Reconstitution depuis audit Ceph

1. Lister les objets sur la période concernée.
2. Reconstituer la chronologie via `timestamp`, `trace_id`, `request_id`.
3. Isoler erreurs et causes (timeout/retry/status code).
4. Produire un rapport post-incident.

## 8) SLO opérationnels et seuils d’alerte (MVP)

- p95 latence sortante > seuil cible pendant 10 min → alerte warning.
- `errors_total / requests_total` > 2% sur fenêtre 5 min → alerte majeure.
- backlog buffer audit croissant > 15 min → alerte critique.
- `circuit_state=open` > 5 min continu → alerte majeure.

## 9) Journal d’incident (template)

- **Incident ID**:
- **Début / Fin**:
- **Impact**:
- **Invariants touchés** (I1..I5):
- **Cause racine**:
- **Actions immédiates**:
- **Actions correctives long terme**:
- **Validation post-fix**:

## 10) RACI minimal

- **On-call SRE**: triage, mitigation immédiate, communication.
- **Ingénieur plateforme**: infrastructure Kafka/Ceph/obs.
- **Responsable applicatif**: correctifs service Rust/résilience.
- **Référent qualité**: vérification conformité invariants et DoD.
