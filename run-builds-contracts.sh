#!/usr/bin/env bash
  set -u  # no -e, ca sa continue
  root="tests/soroban-examples"
  out="wasm_build_locations.txt"
  : > "$out"

  for d in "$root"/*; do
    [ -d "$d" ] || continue
    if [ -f "$d/Cargo.toml" ]; then
      echo "==> $d"
      (cd "$d" && stellar contract build) | tee /tmp/stellar_build.log
      status=${PIPESTATUS[0]}
      if [ $status -ne 0 ]; then
        echo "$d -> BUILD FAILED (exit $status)" | tee -a "$out"
        continue
      fi

      loc=$(rg -o "Wasm File: .*" /tmp/stellar_build.log | sed "s/Wasm File: //")
      if [ -n "$loc" ]; then
        echo "$d -> $loc" | tee -a "$out"
      else
        echo "$d -> (no wasm path found in output)" | tee -a "$out"
      fi
    fi
  done

  echo "Done. Collected locations in $out"
