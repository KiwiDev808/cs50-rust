if cat names1.txt | cargo run Alice Bob Charlie | grep -q 'The winner is Charlie'; then
  echo "[Test 1] Passed"
fi

if cat names2.txt | cargo run Alice Bob Charlie | grep -q 'The winner is Charlie'; then
  echo "[Test 2] Test Passed"
fi