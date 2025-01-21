### Invoke the function locally
total=${1:?Please enter total}
tip=${2:?Please enter tip}
cargo lambda invoke --data-ascii "{\"total\": \"$total\", \"tip\": \"$tip\"}" | jq .

