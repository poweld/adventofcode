set -euo pipefail

day="${1:-}"
if [ -z "${day}" ]; then
  echo "Must pass in a day number" >2
  exit 1
fi

cargo new day${day}
cp -R template/src day${day}/
find day${day}/src -type f -name "*.rs" -print0 | \
  xargs -0 -I {} sed -i "s/<day>/${day}/g" {}

(
  cd day${day}
  ln -s ../../input/day${day} input
)
