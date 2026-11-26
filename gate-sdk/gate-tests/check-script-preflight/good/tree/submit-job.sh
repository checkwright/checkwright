#!/usr/bin/env bash
set -euo pipefail
pkill -P "$parent"
timeout 5 pkill -f worker
pkill -f worker # kill-ok: drains the whole worker pool
echo "pkill -f worker"
# cp out.log /mnt/shared/logs/
cat <<EOF
/mnt/shared is only named in this note
EOF
{ grep x in.log || true; } | head -1
grep x in.log | head -1 # pipefail-ok: the first match is enough
submit --owner admin -u "$USER" job.sh
submit --owner=ops1 job.sh
sbatch <<'EOF'
pkill -P "$job"
EOF
