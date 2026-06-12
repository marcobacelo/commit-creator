[◀️ Página anterior: Troubleshooting](troubleshooting.md)  
[⬅️ Voltar para o README principal](../README.md)  

# 🧭 Roadmap

---

## 🚀 Ideias futuras

* ⚙️ Suporte a arquivo `.commit-pattern.toml`
* 🧩 Tipos customizados por projeto
* 🎨 Emojis customizados
* 🌎 Suporte a múltiplos idiomas
* 📋 Templates de corpo de commit
* 🔗 Integração com issues
* 🧪 Validação de branch name
* 🚀 Releases automatizadas com GitHub Actions
* 🧠 Sugestão automática de commit baseada no diff
* 🧾 Geração de changelog
* 🏷️ Suporte a versionamento semântico
* 🧰 Instalador próprio para Linux/macOS
* 🪟 Suporte dedicado para Windows
* 🧪 Testes de integração com repositórios Git temporários

---

## 🧩 Configuração por projeto

Ideia de arquivo futuro:

```toml
use_emoji = true
max_subject_len = 72
allow_breaking_change = true

[types.feat]
emoji = "✨"
description = "Nova funcionalidade"

[types.fix]
emoji = "🐛"
description = "Correção de bug"

[types.chore]
emoji = "🔧"
description = "Tarefas auxiliares"
```

---

## 🤖 Sugestão baseada no diff

Uma evolução interessante seria permitir:

```bash
commit-pattern commit --suggest
```

A CLI analisaria o diff staged e sugeriria:

* tipo do commit;
* escopo;
* descrição;
* corpo opcional.

---

## 📦 Releases automatizadas

Outra evolução seria usar GitHub Actions para gerar binários automaticamente para:

* Linux x86_64
* Linux ARM64
* macOS Apple Silicon
* macOS Intel
* Windows x86_64

---

## 👑 Visão

A visão do projeto é ser uma CLI simples, rápida e opinativa para padronizar commits em times de desenvolvimento sem depender de ferramentas pesadas.

---

[⬅️ Voltar para o README principal](../README.md)  
