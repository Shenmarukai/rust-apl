#!/usr/bin/env bash
# Semi-automatic pre-1.0 -> Rust 2015 codemods
#
# What it does:
# - Removes legacy crate attrs and extern mods
# - Modernizes some macro names and keywords
# - Converts int/uint -> isize/usize
# - Type-level conversions:
#     ~str      -> String
#     ~[T]      -> Vec<T>
#     ~T        -> Box<T>
# - Expression-level conversions:
#     ~"..."    -> "...".to_string()
#     ~[ ... ]  -> vec![ ... ]  (supports nested brackets)
#
# Notes:
# - This edits files in-place. Commit your work before running.
# - Heuristic: review the diff after running.
# - Remaining ~expr (non-literal) should be replaced with Box::new(expr) manually.

set -euo pipefail
shopt -s nullglob

# Collect .rs files from git (fallback to find if not a git repo)
files=()
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  while IFS= read -r f; do files+=("$f"); done < <(git ls-files '*.rs')
else
  while IFS= read -r -d '' f; do files+=("$f"); done < <(find . -type f -name '*.rs' -print0)
fi

if [ ${#files[@]} -eq 0 ]; then
  echo "No .rs files found"
  exit 0
fi

echo "Applying codemods to ${#files[@]} files..."

apply() { perl -0777 -pe "$1" -i -- "$2"; }

for f in "${files[@]}"; do
  # 1) Remove obsolete crate attributes
  apply 's/\#\s*\[\s*link\s*\([^\)]*\)\s*\]\s*;?\s*\n?//sg' "$f"
  apply 's/\#\s*\[\s*(comment|license)\s*=\s*"[^"]*"\s*\]\s*;?\s*\n?//sg' "$f"
  apply 's/\#\s*\[\s*feature\s*\([^\)]*\)\s*\]\s*;?\s*\n?//sg' "$f"
  apply 's/\#\s*\[\s*main\s*\]\s*\n?//sg' "$f"

  # 2) Remove extern mod
  apply 's/\bextern\s+mod\s+std\s*;\s*\n?//sg' "$f"
  apply 's/\bextern\s+mod\s+extra\s*;\s*\n?//sg' "$f"

  # 3) deriving -> derive
  apply 's/\#\s*\[\s*deriving\s*\(/#[derive(/g' "$f"

  # 4) fail! -> panic!, fmt! -> format!
  apply 's/\bfail!\s*\(/panic!(/g' "$f"
  apply 's/\bfmt!\s*\(/format!(/g' "$f"

  # 5) Remove old priv keyword
  apply 's/\bpriv\s+//g' "$f"

  # 6) int/uint -> isize/usize (review each replacement)
  apply 's/\buint\b/usize/g' "$f"
  apply 's/\bint\b/isize/g' "$f"

  # 7) Owned/unique TYPE conversions (do these before expression-level)
  # ~str -> String
  apply 's/\b~\s*str\b/String/g' "$f"

  # ~[T] -> Vec<T> in common type contexts
  apply 's/(:\s*)~\s*\[\s*([^\]]+?)\s*\]/$1Vec<$2>/g' "$f"
  apply 's/(->\s*)~\s*\[\s*([^\]]+?)\s*\]/$1Vec<$2>/g' "$f"
  apply 's/(,\s*)~\s*\[\s*([^\]]+?)\s*\]/$1Vec<$2>/g' "$f"
  apply 's/(<\s*)~\s*\[\s*([^\]]+?)\s*\]/$1Vec<$2>/g' "$f"
  apply 's/(=\s*)~\s*\[\s*([^\]]+?)\s*\]/$1Vec<$2>/g' "$f"

  # ~T -> Box<T> in common type contexts (after ~str and ~[T])
  apply 's/(:\s*)~\s*([A-Za-z_][A-Za-z0-9_:<>]*)/$1Box<$2>/g' "$f"
  apply 's/(->\s*)~\s*([A-Za-z_][A-Za-z0-9_:<>]*)/$1Box<$2>/g' "$f"
  apply 's/(,\s*)~\s*([A-Za-z_][A-Za-z0-9_:<>]*)/$1Box<$2>/g' "$f"
  apply 's/(<\s*)~\s*([A-Za-z_][A-Za-z0-9_:<>]*)/$1Box<$2>/g' "$f"
  apply 's/(=\s*)~\s*([A-Za-z_][A-Za-z0-9_:<>]*)/$1Box<$2>/g' "$f"

  # 8) Expression-level conversions

  # ~"..." -> "...".to_string()
  # Matches a normal double-quoted literal with escapes; does not match raw strings r"..." or byte strings b"..."
  apply 's/~\s*("([^"\\]|\\.)*")/$1.to_string()/g' "$f"

  # ~[ ... ] -> vec![ ... ]
  # Uses a recursive regex to match balanced nested brackets.
  # This runs after type-level ~[T] -> Vec<T] replacements, so remaining ~[...] should be expressions.
  apply 's/~\s*(\[(?:[^\[\]]+|(?1))*\])/vec!$1/g' "$f"
done

echo "Done.
Next steps:
  1) cargo check
  2) Fix any remaining ~expr (non-literal) uses by hand (Box::new(expr)).
  3) cargo fix --allow-dirty --allow-staged
  4) cargo test
  5) Optionally: cargo clippy --fix -Z unstable-options"
