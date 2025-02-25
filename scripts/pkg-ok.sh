pnpm dlx pkg-ok
declare -i EXIT_CODE=$?

MATCHES=$(find . -type d -path './npm/*' )
while read f _; do
  cd $f 
  echo "Checking $(pnpm pkg get name)"
  pnpm dlx pkg-ok || ((EXIT_CODE=EXIT_CODE+1))
  echo "EXIT_CODE: $EXIT_CODE"
  cd ../..
done < <(find . -type d -path './npm/*')

echo "EXIT_CODE: $EXIT_CODE"
exit $EXIT_CODE