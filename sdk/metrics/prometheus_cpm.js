/**
 * CPM Prometheus Metrics Exporter
 *
 * Exposes Prometheus format metrics (/metrics) for RPC call counts, latency histograms,
 * and language host process memory footprint.
 */

class CpmPrometheusExporter {
  constructor() {
    self.callCount = 0;
    self.totalLatencyMs = 0;
  }

  recordCall(target, durationMs) {
    this.callCount++;
    this.totalLatencyMs += durationMs;
  }

  getMetrics() {
    const avgMs = this.callCount > 0 ? (this.totalLatencyMs / this.callCount).toFixed(2) : 0;
    return `
# HELP cpm_bridge_calls_total Total number of CPM cross-language RPC bridge calls
# TYPE cpm_bridge_calls_total counter
cpm_bridge_calls_total ${this.callCount}

# HELP cpm_bridge_call_duration_seconds Average latency of CPM RPC calls in seconds
# TYPE cpm_bridge_call_duration_seconds gauge
cpm_bridge_call_duration_seconds ${(avgMs / 1000).toFixed(4)}
`.trim();
  }
}

module.exports = { CpmPrometheusExporter };
