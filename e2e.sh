#!/usr/bin/env bash
# =============================================================================
# Testes E2E — alea-jact-est
# =============================================================================
# OBS: propositalmente SEM `set -e`. Um test runner não pode abortar no
# primeiro comando que falhar — comandos "falhando" (exit != 0) fazem parte
# do que estamos testando (ex: bloco 10, entradas inválidas). O script só
# decide seu próprio exit code no final, baseado na contagem PASS/FAIL.
set -uo pipefail

# ---------------------------------------------------------------------------
# Utilitários
# ---------------------------------------------------------------------------
PASS=0
FAIL=0
CLI="${CLI_PATH:-alea-jact-est}"

green() { printf '\033[0;32m✔ %s\033[0m\n' "$*"; }
red() { printf '\033[0;31m✘ %s\033[0m\n' "$*"; }
bold() { printf '%b\n' "\033[1m$*\033[0m"; }

pass() {
  green "$1"
  ((PASS++))
}
fail() {
  red "$1"
  ((FAIL++))
}

# assert_exit_code <esperado> <real> <label>
assert_exit() {
  local expected=$1 actual=$2 label=$3
  if [[ "$actual" -eq "$expected" ]]; then
    pass "$label"
  else
    fail "$label — exit code esperado=$expected, obtido=$actual"
  fi
}

# assert_match <regex> <string> <label>
assert_match() {
  local regex=$1 value=$2 label=$3
  if echo "$value" | grep -qE "$regex"; then
    pass "$label"
  else
    fail "$label — esperado match /$regex/, obtido: '$value'"
  fi
}

# assert_not_match <regex> <string> <label>
assert_not_match() {
  local regex=$1 value=$2 label=$3
  if echo "$value" | grep -qE "$regex"; then
    fail "$label — não deveria conter /$regex/, obtido: '$value'"
  else
    pass "$label"
  fi
}

# assert_line_count <esperado> <string> <label>
assert_lines() {
  local expected=$1 value=$2 label=$3
  local actual
  actual=$(echo "$value" | grep -c .)
  if [[ "$actual" -eq "$expected" ]]; then
    pass "$label"
  else
    fail "$label — esperado $expected linhas, obtido $actual"
  fi
}

# assert_length <esperado> <string> <label>
assert_length() {
  local expected=$1 value=$2 label=$3
  local actual=${#value}
  if [[ "$actual" -eq "$expected" ]]; then
    pass "$label"
  else
    fail "$label — esperado comprimento=$expected, obtido=$actual"
  fi
}

# ---------------------------------------------------------------------------
# Verificação do binário
# ---------------------------------------------------------------------------
bold "=== Verificando binário ==="
if ! command -v "$CLI" &>/dev/null; then
  red "Binário '$CLI' não encontrado. Compile com 'cargo build --release' e adicione ao PATH,"
  red "ou defina CLI_PATH=/caminho/para/alea-jact-est antes de rodar o script."
  exit 1
fi
pass "Binário encontrado: $(command -v "$CLI")"

# ---------------------------------------------------------------------------
# Bloco 1 — Comportamento padrão
# ---------------------------------------------------------------------------
bold "\n=== 1. Comportamento padrão ==="

OUT=$("$CLI")
CODE=$?
assert_exit 0 "$CODE" "1.1 exit code 0 sem argumentos"
assert_length 5 "$OUT" "1.2 comprimento padrão = 5"
assert_match '^[A-Za-z0-9]+$' "$OUT" "1.3 charset alfanumérico padrão"

# ---------------------------------------------------------------------------
# Bloco 2 — Flag --length / -l
# ---------------------------------------------------------------------------
bold "\n=== 2. --length / -l ==="

for LEN in 1 8 16 64; do
  OUT=$("$CLI" -l "$LEN")
  assert_length "$LEN" "$OUT" "2.x comprimento=$LEN"
done

OUT=$("$CLI" --length 12)
assert_length 12 "$OUT" "2.5 --length longo"

# ---------------------------------------------------------------------------
# Bloco 3 — Flag --count / -n
# ---------------------------------------------------------------------------
bold "\n=== 3. --count / -n ==="

for N in 1 3 5; do
  OUT=$("$CLI" -n "$N")
  assert_lines "$N" "$OUT" "3.x -n $N linhas"
done

OUT=$("$CLI" --count 4)
assert_lines 4 "$OUT" "3.4 --count 4 linhas"

# Cada linha deve ter comprimento padrão (5)
while IFS= read -r line; do
  assert_length 5 "$line" "3.5 comprimento de cada linha com -n 3"
done < <("$CLI" -n 3)

# ---------------------------------------------------------------------------
# Bloco 4 — --charset numeric
# ---------------------------------------------------------------------------
bold "\n=== 4. --charset numeric ==="

OUT=$("$CLI" -l 20 --charset numeric)
CODE=$?
assert_exit 0 "$CODE" "4.1 exit code 0"
assert_length 20 "$OUT" "4.2 comprimento=20"
assert_match '^[0-9]+$' "$OUT" "4.3 apenas dígitos"
assert_not_match '[A-Za-z]' "$OUT" "4.4 sem letras"

# ---------------------------------------------------------------------------
# Bloco 5 — --charset alphabetic
# ---------------------------------------------------------------------------
bold "\n=== 5. --charset alphabetic ==="

OUT=$("$CLI" -l 20 --charset alphabetic)
assert_length 20 "$OUT" "5.1 comprimento=20"
assert_match '^[A-Za-z]+$' "$OUT" "5.2 apenas letras"
assert_not_match '[0-9]' "$OUT" "5.3 sem dígitos"

# ---------------------------------------------------------------------------
# Bloco 6 — --uppercase
# ---------------------------------------------------------------------------
bold "\n=== 6. --uppercase ==="

OUT=$("$CLI" -l 20 --charset alphabetic --uppercase)
assert_match '^[A-Z]+$' "$OUT" "6.1 apenas maiúsculas"
assert_not_match '[a-z]' "$OUT" "6.2 sem minúsculas"

OUT=$("$CLI" -l 20 --charset alphanumeric --uppercase)
assert_match '^[A-Z0-9]+$' "$OUT" "6.3 maiúsculas + dígitos"

# ---------------------------------------------------------------------------
# Bloco 7 — --lowercase
# ---------------------------------------------------------------------------
bold "\n=== 7. --lowercase ==="

OUT=$("$CLI" -l 20 --charset alphabetic --lowercase)
assert_match '^[a-z]+$' "$OUT" "7.1 apenas minúsculas"
assert_not_match '[A-Z]' "$OUT" "7.2 sem maiúsculas"

OUT=$("$CLI" -l 20 --charset alphanumeric --lowercase)
assert_match '^[a-z0-9]+$' "$OUT" "7.3 minúsculas + dígitos"

# ---------------------------------------------------------------------------
# Bloco 8 — Combinações: -l + -n + charset
# ---------------------------------------------------------------------------
bold "\n=== 8. Combinações ==="

OUT=$("$CLI" -l 8 -n 3 --charset numeric)
assert_lines 3 "$OUT" "8.1 3 linhas"
while IFS= read -r line; do
  assert_length 8 "$line" "8.2 cada linha tem comprimento 8"
  assert_match '^[0-9]+$' "$line" "8.3 cada linha só tem dígitos"
done <<<"$OUT"

OUT=$("$CLI" --length 10 --count 5 --charset alphabetic --uppercase)
assert_lines 5 "$OUT" "8.4 5 linhas uppercase"
while IFS= read -r line; do
  assert_length 10 "$line" "8.5 comprimento 10"
  assert_match '^[A-Z]+$' "$line" "8.6 apenas maiúsculas"
done <<<"$OUT"

# ---------------------------------------------------------------------------
# Bloco 9 — Aleatoriedade básica
# ---------------------------------------------------------------------------
bold "\n=== 9. Aleatoriedade ==="

A=$("$CLI" -l 16)
B=$("$CLI" -l 16)
if [[ "$A" != "$B" ]]; then
  pass "9.1 duas execuções produzem resultados diferentes"
else
  # Colisão é estatisticamente possível mas muito improvável com l=16
  fail "9.1 duas execuções produziram o mesmo resultado (possível bug)"
fi

# ---------------------------------------------------------------------------
# Bloco 10 — Entradas inválidas (exit code != 0)
# ---------------------------------------------------------------------------
bold "\n=== 10. Entradas inválidas ==="

set +e # não abortar no erro esperado

"$CLI" --charset invalido &>/dev/null
assert_exit 2 $? "10.1 charset inválido retorna erro"

"$CLI" -l 0 &>/dev/null
# length=0 pode ser erro ou string vazia — ajuste conforme o comportamento real
CODE=$?
if [[ $CODE -ne 0 ]]; then
  pass "10.2 -l 0 retorna erro"
else
  OUT=$("$CLI" -l 0)
  assert_length 0 "$OUT" "10.2 -l 0 retorna string vazia"
fi

"$CLI" --uppercase --lowercase &>/dev/null
assert_exit 2 $? "10.3 --uppercase + --lowercase juntos retornam erro"

"$CLI" --opcao-inexistente &>/dev/null
assert_exit 2 $? "10.4 flag desconhecida retorna erro"

set -e

# ---------------------------------------------------------------------------
# Resumo
# ---------------------------------------------------------------------------
TOTAL=$((PASS + FAIL))
bold "\n=== Resultado ==="
printf "Total: %d | " "$TOTAL"
green "Passou: $PASS"
if [[ $FAIL -gt 0 ]]; then
  red "Falhou: $FAIL"
  exit 1
else
  printf '\033[0;32mTodos os testes passaram!\033[0m\n'
fi
