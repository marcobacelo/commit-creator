[◀️ Página anterior: Como usar no dia a dia](daily-usage.md)  
[⬅️ Voltar para o README principal](../README.md)  

# 🧾 Todos os comandos disponíveis

---

## 📌 Ajuda geral

```bash
commit-pattern --help
```

Mostra a ajuda principal da CLI.

---

## 🧙 `commit-pattern commit`

Abre o assistente interativo e executa `git commit`.

```bash
commit-pattern commit
```

Usa apenas arquivos que já estão staged.

Fluxo recomendado:

```bash
git add src/main.rs
commit-pattern commit
```

---

## ⚡ `commit-pattern commit --all`

Executa `git add .` antes do assistente.

```bash
commit-pattern commit --all
```

Equivale a:

```bash
git add .
git commit -m "mensagem gerada pela CLI"
```

---

## 🧪 `commit-pattern commit --dry-run`

Gera a mensagem sem criar commit.

```bash
commit-pattern commit --dry-run
```

---

## ✅ `commit-pattern commit --yes`

Pula a confirmação final.

```bash
commit-pattern commit --yes
```

Combinação possível:

```bash
commit-pattern commit --all --yes
```

---

## 🔎 `commit-pattern validate --message`

Valida uma mensagem diretamente.

```bash
commit-pattern validate --message "feat(auth): adicionar login"
```

Com emoji:

```bash
commit-pattern validate --message "✨ feat(auth): adicionar login social"
```

---

## 📄 `commit-pattern validate --file`

Valida mensagem a partir de um arquivo.

```bash
commit-pattern validate --file .git/COMMIT_EDITMSG
```

Uso típico no hook `commit-msg`:

```sh
#!/bin/sh
commit-pattern validate --file "$1"
```

---

## 🔢 `commit-pattern validate --max-4-words`

Exige no máximo 4 palavras na descrição da primeira linha.

```bash
commit-pattern validate --message "feat: adicionar login social" --max-4-words
```

---

## 📏 `commit-pattern validate --max-subject-len`

Define tamanho máximo da primeira linha.

```bash
commit-pattern validate --message "feat: adicionar login" --max-subject-len 72
```

---

## 🏷️ `commit-pattern list-types`

Lista os tipos aceitos.

```bash
commit-pattern list-types
```

---

## 🪝 `commit-pattern install-hook`

Instala hook local.

```bash
commit-pattern install-hook
```

Cria:

```txt
.git/hooks/commit-msg
```

Esse hook não é versionado.

---

## 🪝 `commit-pattern install-hook --shared`

Instala hook compartilhado.

```bash
commit-pattern install-hook --shared
```

Cria:

```txt
.githooks/commit-msg
```

Configura:

```bash
git config core.hooksPath .githooks
```

Essa opção é recomendada para times.

---

## 🧰 `commit-pattern setup-shell`

Adiciona `~/.cargo/bin` ao `PATH`.

```bash
commit-pattern setup-shell
```

Suporta:

* `bash`
* `zsh`
* `fish`

Depois rode `source` no arquivo indicado pela CLI ou abra um novo terminal.

---

## 🔨 `commit-pattern build-help`

Mostra instruções de build e distribuição.

```bash
commit-pattern build-help
```

---

## 📖 `commit-pattern manual`

Mostra manual completo no terminal.

```bash
commit-pattern manual
```

## 🚀 Próximo passo

Valide o uso da CLI:

[➡️ Próxima: Validação aplicada](validation.md)

---

[◀️ Página anterior: Como usar no dia a dia](daily-usage.md)  
[⬅️ Voltar para o README principal](../README.md)  
