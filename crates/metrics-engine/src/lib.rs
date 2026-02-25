use std::collections::{HashMap, VecDeque};
use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};

use domain::{MetricsRecorder, RequestMetricPoint};
use tracing::warn;

// ---------------------------------------------------------------------------
// Label clé : dimensions conservées pour respecter l'invariant I5 (NAT)
// ---------------------------------------------------------------------------

/// Clé d'agrégation par route, IP source (interne) et IP destination (externe).
///
/// Conserver les deux dimensions IP permet de localiser un nœud dégradé
/// (via `internal_ip`) et d'identifier la destination problématique
/// (via `external_ip`), même après traduction NAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetricLabel {
    pub route_group: String,
    pub internal_ip: String,
    pub external_ip: String,
}

// ---------------------------------------------------------------------------
// Fenêtre glissante de latences (horloge monotone — invariant I3)
// ---------------------------------------------------------------------------

/// Fenêtre bornée de mesures de latence pour le calcul exact de percentiles.
///
/// Les mesures sont produites via `std::time::Instant` (horloge monotone),
/// ce qui garantit l'absence de valeurs négatives en cas de correction NTP.
/// La fenêtre évince les échantillons les plus anciens quand `max_samples`
/// est atteint — comportement sliding window.
#[derive(Debug)]
struct LatencyWindow {
    samples: VecDeque<u64>,
    max_samples: usize,
}

impl LatencyWindow {
    fn new(max_samples: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Ajoute un échantillon en évincant le plus ancien si nécessaire.
    fn push(&mut self, latency_ms: u64) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(latency_ms);
    }

    /// Calcule le p-ième percentile (0–100) par tri exact.
    ///
    /// Retourne `None` si la fenêtre est vide.
    fn percentile(&self, p: f64) -> Option<u64> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted: Vec<u64> = self.samples.iter().copied().collect();
        sorted.sort_unstable();
        let idx = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
        Some(sorted[idx.saturating_sub(1).min(sorted.len() - 1)])
    }
}

// ---------------------------------------------------------------------------
// Moteur de métriques principal
// ---------------------------------------------------------------------------

/// Moteur de métriques en mémoire : compteurs atomiques + histogrammes de latence.
///
/// Thread-safe : les compteurs utilisent des atomics (`Relaxed` suffisant pour
/// des incréments indépendants), la map de latences est protégée par un `Mutex`.
#[derive(Debug)]
pub struct InMemoryMetricsEngine {
    requests_total: AtomicU64,
    errors_total: AtomicU64,
    retries_total: AtomicU64,
    timeouts_total: AtomicU64,
    /// Fenêtres de latence indexées par label (route + IPs).
    latency_windows: Mutex<HashMap<MetricLabel, LatencyWindow>>,
    max_latency_samples: usize,
}

impl InMemoryMetricsEngine {
    /// Crée un moteur avec une taille de fenêtre glissante configurable.
    ///
    /// `max_latency_samples` doit correspondre à `MetricsConfig::max_latency_samples`
    /// pour garantir la cohérence entre environnements (invariant I3).
    pub fn new(max_latency_samples: usize) -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            errors_total: AtomicU64::new(0),
            retries_total: AtomicU64::new(0),
            timeouts_total: AtomicU64::new(0),
            latency_windows: Mutex::new(HashMap::new()),
            max_latency_samples,
        }
    }

    /// Produit un snapshot instantané des métriques agrégées.
    pub fn snapshot(&self) -> NetworkMetricsSnapshot {
        let latency_by_label = match self.latency_windows.lock() {
            Ok(windows) => windows
                .iter()
                .map(|(label, window)| LatencyPercentiles {
                    label: MetricLabelSnapshot {
                        route_group: label.route_group.clone(),
                        internal_ip: label.internal_ip.clone(),
                        external_ip: label.external_ip.clone(),
                    },
                    p50_ms: window.percentile(50.0),
                    p95_ms: window.percentile(95.0),
                    p99_ms: window.percentile(99.0),
                    sample_count: window.samples.len(),
                })
                .collect(),
            Err(err) => {
                warn!(%err, "latency_windows lock poisoned during snapshot");
                vec![]
            }
        };

        NetworkMetricsSnapshot {
            requests_total: self.requests_total.load(Ordering::Relaxed),
            errors_total: self.errors_total.load(Ordering::Relaxed),
            retries_total: self.retries_total.load(Ordering::Relaxed),
            timeouts_total: self.timeouts_total.load(Ordering::Relaxed),
            latency_by_label,
        }
    }
}

impl MetricsRecorder for InMemoryMetricsEngine {
    /// Enregistre un point de mesure : incrémente les compteurs et
    /// ajoute la latence dans la fenêtre du label correspondant.
    fn record(&self, point: &RequestMetricPoint) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.retries_total
            .fetch_add(u64::from(point.retry_count), Ordering::Relaxed);
        self.timeouts_total
            .fetch_add(u64::from(point.timeout_count), Ordering::Relaxed);

        if point.is_error {
            self.errors_total.fetch_add(1, Ordering::Relaxed);
        }

        let label = MetricLabel {
            route_group: point.route_group.clone(),
            internal_ip: point.internal_ip.clone(),
            external_ip: point.external_ip.clone(),
        };

        match self.latency_windows.lock() {
            Ok(mut windows) => {
                windows
                    .entry(label)
                    .or_insert_with(|| LatencyWindow::new(self.max_latency_samples))
                    .push(point.latency_ms);
            }
            Err(err) => {
                warn!(%err, "latency_windows lock poisoned during record");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Types de snapshot exposés à l'API
// ---------------------------------------------------------------------------

/// Snapshot des percentiles de latence pour un label donné.
#[derive(Debug, Clone)]
pub struct LatencyPercentiles {
    pub label: MetricLabelSnapshot,
    pub p50_ms: Option<u64>,
    pub p95_ms: Option<u64>,
    pub p99_ms: Option<u64>,
    pub sample_count: usize,
}

/// Version sérialisable du label de métriques.
#[derive(Debug, Clone)]
pub struct MetricLabelSnapshot {
    pub route_group: String,
    pub internal_ip: String,
    pub external_ip: String,
}

/// Snapshot complet exposé via `GET /v1/metrics/network`.
#[derive(Debug, Clone)]
pub struct NetworkMetricsSnapshot {
    pub requests_total: u64,
    pub errors_total: u64,
    pub retries_total: u64,
    pub timeouts_total: u64,
    /// Percentiles de latence par combinaison (route, internal_ip, external_ip).
    pub latency_by_label: Vec<LatencyPercentiles>,
}
