# Alea Jact Est

*"Alea iacta est"* — A frase latina dita por Júlio César ao cruzar o Rubicão, significando **"A sorte está lançada"**. Um nome apropriado para um gerador de strings aleatórias.

## Sobre

CLI simples em Rust que gera strings aleatórias de tamanho configurável, com suporte a múltiplos charsets.

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Instalação

```bash
cargo install --path .
```

## Uso

```bash
# Gerar string de 5 caracteres alfanuméricos (padrão)
alea-jact-est

# Gerar string de 12 caracteres numéricos
alea-jact-est -l 12 --charset numeric

# Gerar 3 strings de 8 caracteres alfabéticos
alea-jact-est --length 8 -n 3 --charset alphabetic

# Gerar string só com maiúsculas
alea-jact-est -l 10 --charset alphabetic --uppercase

# Gerar string só com minúsculas
alea-jact-est -l 10 --charset alphabetic --lowercase
```

### Opções

| Opção | Descrição | Padrão |
|-------|-----------|--------|
| `-l`, `--length` | Define o tamanho da string gerada | `5` |
| `-n`, `--count` | Número de strings a gerar | `1` |
| `--charset` | Define o conjunto de caracteres (`alphanumeric`, `numeric`, `alphabetic`) | `alphanumeric` |
| `--uppercase` | Usa apenas letras maiúsculas | `false` |
| `--lowercase` | Usa apenas letras minúsculas | `false` |

## Exemplo de saída

```
$ alea-jact-est -l 8
xK9mZp2Q
```

## Desenvolvimento

```bash
# Compilar (debug)
cargo build

# Compilar (release/otimizado)
cargo build --release

# Rodar
cargo run -- [OPÇÕES]
```

## Stack

- **clap** — Parsing de argumentos de linha de comando
- **rand** — Geração de números/bytes aleatórios
- **thiserror** — Tratamento de erros customizados

## Licença

MIT
