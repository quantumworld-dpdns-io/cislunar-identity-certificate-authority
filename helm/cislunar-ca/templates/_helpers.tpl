{{- define "cislunar-ca.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "cislunar-ca.labels" -}}
helm.sh/chart: {{ include "cislunar-ca.name" . }}-{{ .Chart.Version }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/part-of: cislunar-ca
{{- end }}
