#!/usr/bin/env bash
set -euo pipefail
pkill -f worker
pkill -P "$parent" -9
cp out.log /mnt/shared/logs/
grep x in.log | head -1
submit --owner Admin job.sh
submit -u=x job.sh
pkill -f worker # kill-ok: why
sbatch <<'EOF'
pkill -u me
EOF
sbatch --wrap "ls /scratch/team"
