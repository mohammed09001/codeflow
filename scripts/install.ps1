param([string]$Prefix = "$env:LOCALAPPDATA\CodeFlow")
$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $Prefix | Out-Null
Write-Output "CodeFlow install prefix: $Prefix"
Write-Output "Optional analyzers (SCIP/Joern) are capability-gated and may be installed separately."
