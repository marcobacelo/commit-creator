[◀️ Página anterior: Como distribuir para colegas](distribution.md) • [⬅️ Voltar para o README principal](../README.md) • [➡️ Próxima: Todos os comandos disponíveis](commands.md)

# 🧙 Como usar no dia a dia

---

## 📌 Fluxo recomendado

O fluxo mais seguro é adicionar manualmente os arquivos e depois abrir o assistente:

```bash
git add src/main.rs
commit-pattern commit
```

Assim você controla exatamente o que será commitado.

---

## 🧙 Assistente interativo

Execute:

```bash
commit-pattern commit
```

A CLI vai perguntar:

* tipo do commit;
* se deseja usar emoji;
* escopo opcional;
* descrição curta;
* se é breaking change;
* corpo opcional;
* footer opcional;
* confirmação final.

---

## ⚡ Adicionar tudo automaticamente

Para executar `git add .` antes do commit:

```bash
commit-pattern commit --all
```

Esse comando equivale a:

```bash
git add .
git commit -m "mensagem gerada pela CLI"
```

> Observação: o comando usa `git add .`, não `git add -A`.

---

## 🧪 Testar sem commitar

Para gerar a mensagem sem criar commit:

```bash
commit-pattern commit --dry-run
```

---

## ✅ Pular confirmação final

```bash
commit-pattern commit --yes
```

Também é possível combinar:

```bash
commit-pattern commit --all --yes
```

---

## 🔎 Validar mensagem manualmente

```bash
commit-pattern validate --message "feat(auth): adicionar login"
```

Com emoji:

```bash
commit-pattern validate --message "✨ feat(auth): adicionar login social"
```

Com breaking change:

```bash
commit-pattern validate --message "feat(api)!: alterar contrato"
```

---

## 🪝 Usando com hooks

Instale o hook compartilhado:

```bash
commit-pattern install-hook --shared
```

Depois versionar:

```bash
git add .githooks
commit-pattern commit
```

---

## 🧯 Problemas comuns

Se encontrar algum problema, consulte:

[🧯 Troubleshooting](troubleshooting.md)

---

## 🚀 Próximo passo

Veja todos os comandos disponíveis:

[➡️ Próxima: Todos os comandos disponíveis](commands.md)

---

[◀️ Página anterior: Como distribuir para colegas](distribution.md)
[⬅️ Voltar para o README principal](../README.md)
