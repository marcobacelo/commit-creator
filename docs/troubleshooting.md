[⬅️ Voltar para o README principal](../README.md)

# 🧯 Troubleshooting

---

## 📌 Visão geral

Este documento reúne problemas comuns ao instalar, compilar e usar o **Commit Pattern CLI**.

---

## ❌ Comando `commit-pattern` não encontrado

### Sintoma

Ao executar:

```bash
commit-pattern --help
```

Você recebe algo como:

```txt
command not found: commit-pattern
```

ou:

```txt
commit-pattern: comando não encontrado
```

---

### Possíveis causas

* O binário não foi instalado.
* O binário foi instalado em um diretório que não está no `PATH`.
* O terminal ainda não foi recarregado após configurar o shell.

---

### Solução usando instalação global

Se você compilou o projeto com:

```bash
cargo build --release
```

Instale o binário com:

```bash
sudo install -m 755 target/release/commit-pattern /usr/local/bin/commit-pattern
```

Depois teste:

```bash
commit-pattern --help
```

---

### Solução usando `~/.cargo/bin`

Se você instalou com Cargo:

```bash
cargo install --path . --force
```

Configure o shell:

```bash
commit-pattern setup-shell
```

Depois recarregue o shell.

Para `bash`:

```bash
source ~/.bashrc
```

Para `zsh`:

```bash
source ~/.zshrc
```

Para `fish`:

```bash
source ~/.config/fish/config.fish
```

---

## ❌ Não há alterações staged

### Sintoma

Ao executar:

```bash
commit-pattern commit
```

Você recebe:

```txt
não há alterações staged para commitar
```

---

### Causa

O comando `commit-pattern commit` usa somente arquivos que já estão em stage.

---

### Solução 1: adicionar arquivos manualmente

```bash
git add src/main.rs
commit-pattern commit
```

---

### Solução 2: usar `--all`

```bash
commit-pattern commit --all
```

Esse comando executa:

```bash
git add .
```

Antes de abrir o assistente interativo.

---

## ❌ Hook bloqueou meu commit

### Sintoma

Ao executar `git commit`, o hook `commit-msg` bloqueia a mensagem.

---

### Causa

A mensagem não segue o padrão esperado:

```txt
tipo: descrição
```

ou:

```txt
tipo(escopo): descrição
```

ou:

```txt
emoji tipo(escopo): descrição
```

---

### Solução recomendada

Use o assistente interativo:

```bash
commit-pattern commit
```

---

### Solução manual

Valide sua mensagem antes de commitar:

```bash
commit-pattern validate --message "feat(auth): adicionar login"
```

---

## ❌ Erro ao instalar hook compartilhado

### Sintoma

Ao executar:

```bash
commit-pattern install-hook --shared
```

O comando falha.

---

### Possíveis causas

* O diretório atual não é um repositório Git.
* O Git não está instalado.
* O usuário não tem permissão de escrita no diretório atual.

---

### Verificações

Confirme que está dentro de um repositório Git:

```bash
git rev-parse --is-inside-work-tree
```

A saída esperada é:

```txt
true
```

Confirme que o Git está instalado:

```bash
git --version
```

---

## ❌ O hook compartilhado não está sendo executado

### Sintoma

O arquivo `.githooks/commit-msg` existe, mas commits inválidos não são bloqueados.

---

### Causa provável

O Git ainda não está configurado para usar `.githooks`.

---

### Solução

Execute:

```bash
git config core.hooksPath .githooks
```

Depois valide:

```bash
git config core.hooksPath
```

A saída esperada é:

```txt
.githooks
```

---

## ❌ Permissão negada ao executar o hook

### Sintoma

Ao commitar, aparece erro de permissão no hook.

---

### Solução

Dê permissão de execução:

```bash
chmod +x .githooks/commit-msg
```

Ou reinstale o hook:

```bash
commit-pattern install-hook --shared
```

---

## ❌ Erro ao compilar o projeto

### Sintoma

Ao executar:

```bash
cargo build --release
```

A compilação falha.

---

### Verificações

Confirme que Rust e Cargo estão instalados:

```bash
rustc --version
cargo --version
```

Atualize a toolchain:

```bash
rustup update
```

Limpe artefatos antigos:

```bash
cargo clean
```

Compile novamente:

```bash
cargo build --release
```

---

## ❌ `cargo install --path .` falha com `no targets specified`

### Sintoma

Erro parecido com:

```txt
no targets specified in the manifest
either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```

---

### Causa

O Cargo não encontrou um alvo executável no projeto.

---

### Solução

Garanta que exista:

```txt
src/main.rs
```

E que o `Cargo.toml` contenha:

```toml
[[bin]]
name = "commit-pattern"
path = "src/main.rs"
```

---

## ✅ Checklist final

Se algo não funcionar, valide:

* [ ] Estou dentro de um repositório Git?
* [ ] O Rust está instalado?
* [ ] O Cargo está instalado?
* [ ] O Git está instalado?
* [ ] O binário `commit-pattern` está no `PATH`?
* [ ] O hook tem permissão de execução?
* [ ] O `core.hooksPath` aponta para `.githooks`?
* [ ] Há arquivos staged antes do commit?
