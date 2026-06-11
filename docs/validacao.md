# 🛡️ Validação aplicada

[⬅️ Voltar para o README principal](../README.md)

---

## 📌 O que a CLI valida

A CLI valida:

* ✅ tipo obrigatório;
* ✅ descrição obrigatória;
* ✅ tipos permitidos;
* ✅ escopo não vazio quando informado;
* ✅ suporte a emoji;
* ✅ suporte a breaking change;
* ✅ limite de tamanho da primeira linha;
* ✅ formato `tipo: descrição`;
* ✅ formato `tipo(escopo): descrição`;
* ✅ formato `emoji tipo(escopo): descrição`.

---

## ✅ Exemplos válidos

```txt
feat: adicionar login
fix(api): corrigir timeout
✨ feat(auth): adicionar login social
🐛 fix(api): corrigir timeout
feat(api)!: alterar contrato
```

---

## ❌ Exemplos inválidos

```txt
feature: adicionar login
feat adicionar login
feat:
feat(): adicionar login
FIX: corrigir bug
```

---

## 🧱 Formato esperado

```txt
emoji tipo(escopo): descrição
```

O emoji é opcional.

O escopo é opcional.

A descrição é obrigatória.

---

## 🏷️ Tipos aceitos

| Tipo       | Descrição                                 |
| ---------- | ----------------------------------------- |
| `feat`     | Nova funcionalidade                       |
| `fix`      | Correção de bug                           |
| `docs`     | Documentação                              |
| `test`     | Testes                                    |
| `build`    | Build, dependências ou empacotamento      |
| `perf`     | Melhoria de performance                   |
| `style`    | Formatação, estilo ou lint                |
| `refactor` | Refatoração sem alterar comportamento     |
| `chore`    | Tarefas auxiliares                        |
| `ci`       | Integração contínua                       |
| `raw`      | Arquivos raw, configs ou dados            |
| `cleanup`  | Limpeza de código                         |
| `remove`   | Remoção de código, arquivo ou dependência |
| `init`     | Commit inicial                            |

---

## 💥 Breaking changes

A CLI aceita breaking changes no cabeçalho:

```txt
feat(api)!: alterar contrato
```

Também pode gerar footer:

```txt
BREAKING CHANGE: alterar contrato
```

---

## 🪝 Validação em hook

O hook recomendado é o `commit-msg`.

Exemplo:

```sh
#!/bin/sh
commit-pattern validate --file "$1"
```

Esse hook bloqueia commits fora do padrão.

---

[⬅️ Voltar para o README principal](../README.md)
