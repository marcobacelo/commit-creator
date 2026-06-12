# 🚀 Commit Pattern CLI

<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" alt="Rust Logo" width="140" />
</p>

<p align="center">
  <strong>CLI interativo em Rust para criar commits padronizados com Conventional Commits + emojis.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-CLI-orange?logo=rust&logoColor=white" alt="Rust CLI" />
  <img src="https://img.shields.io/badge/Conventional%20Commits-enabled-blue" alt="Conventional Commits" />
  <img src="https://img.shields.io/badge/Git%20Hooks-supported-brightgreen" alt="Git Hooks Supported" />
  <img src="https://img.shields.io/badge/Made%20with-%F0%9F%92%9C-purple" alt="Made with purple heart" />
</p>

---

## 📌 Sobre o projeto

O **Commit Pattern CLI** é uma aplicação de terminal feita em **Rust** para ajudar desenvolvedores a criarem commits padronizados, consistentes e fáceis de entender.

A ferramenta abre um assistente interativo no terminal, pergunta como deve ser o commit e executa o `git commit` automaticamente.

Ela também pode validar mensagens de commit e instalar hooks Git para impedir commits fora do padrão.

---

## ✨ Funcionalidades

* 🧙 Assistente interativo para criação de commits
* 🧾 Geração automática de mensagens padronizadas
* 🦀 Aplicação compilada em Rust
* ✅ Validação de mensagens de commit
* 🪝 Instalação de Git hooks
* 📦 Suporte a distribuição por binário compilado
* 🧰 Suporte a `bash`, `zsh` e `fish`
* 🔥 Suporte a commits com emoji
* 💥 Suporte a breaking changes
* 🧪 Modo `dry-run`
* 🚀 Comando para configurar shell automaticamente

---

## 🧱 Padrão de commit

A CLI gera mensagens no seguinte formato:

```txt
emoji tipo(escopo): descrição
```

Exemplos:

```txt
✨ feat(auth): adicionar login social
🐛 fix(api): corrigir timeout
📝 docs: atualizar README
♻️ refactor(service): simplificar validação
🔥 remove: remover código obsoleto
```

O emoji e o escopo são opcionais.

Exemplos também válidos:

```txt
feat: adicionar login
fix(api): corrigir timeout
docs: atualizar README
```

---

## 🏷️ Tipos de commit disponíveis

| Emoji | Tipo       | Descrição                                 |
| ----- | ---------- | ----------------------------------------- |
| ✨     | `feat`     | Nova funcionalidade                       |
| 🐛    | `fix`      | Correção de bug                           |
| 📝    | `docs`     | Documentação                              |
| ✅     | `test`     | Testes                                    |
| 📦    | `build`    | Build, dependências ou empacotamento      |
| ⚡     | `perf`     | Melhoria de performance                   |
| 💄    | `style`    | Formatação, estilo ou lint                |
| ♻️    | `refactor` | Refatoração sem alterar comportamento     |
| 🔧    | `chore`    | Tarefas auxiliares                        |
| 👷    | `ci`       | Integração contínua                       |
| 🗃️   | `raw`      | Arquivos raw, configs ou dados            |
| 🧹    | `cleanup`  | Limpeza de código                         |
| 🔥    | `remove`   | Remoção de código, arquivo ou dependência |
| 🎉    | `init`     | Commit inicial                            |

---

## 📚 Documentação

As seções detalhadas foram separadas em arquivos próprios:

* [🧰 Pré-requisitos](docs/prerequisites.md)
* [🦀 Instalando Rust](docs/installing-rust.md)
* [👥 Como distribuir para colegas](docs/distribution.md)
* [🧙 Como usar no dia a dia](docs/daily-usage.md)
* [🧾 Todos os comandos disponíveis](docs/commands.md)
* [🛡️ Validação aplicada](docs/validation.md)
* [🧯 Troubleshooting](docs/troubleshooting.md)
* [🧭 Roadmap](docs/roadmap.md)

---

## 📁 Estrutura do projeto

```txt
commit-creator/
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── README.md
├── docs/
│   ├── prerequisites.md
│   ├── installing-rust.md
│   ├── distribution.md
│   ├── daily-usage.md
│   ├── commands.md
│   ├── validation.md
│   ├── troubleshooting.md
│   └── roadmap.md
└── src/
    └── main.rs
```

---

## 🖥️ Instalação Local

Clone o projeto:

```bash
git clone <url-do-repositorio>
cd commit-creator
```

Compile em modo release:

```bash
cargo build --release
```

Instale o binário globalmente na máquina:

```bash
sudo install -m 755 target/release/commit-pattern /usr/local/bin/commit-pattern
```

Valide a instalação:

```bash
commit-pattern --help
```

Ou:

```bash
commit-pattern list-types
```

---

## 🪝 Git hooks

Instale o hook compartilhado:

```bash
commit-pattern install-hook --shared
```

Isso cria:

```txt
.githooks/commit-msg
```

E configura:

```bash
git config core.hooksPath .githooks
```

Depois versionar:

```bash
git add .githooks
git commit -m "chore: adicionar hook de commit"
```

---

## 🦀 Feito com Rust

<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" alt="Rust Logo" width="180" />
</p>

Este projeto usa Rust por ser uma linguagem excelente para criação de CLIs:

* ⚡ Performance alta
* 🛡️ Segurança de memória
* 📦 Binários fáceis de distribuir
* 🧰 Ecossistema forte para ferramentas de terminal
* 🚀 Boa experiência para automações locais

---

## 🤝 Contribuição

Sugestões, melhorias e correções são bem-vindas.

Fluxo sugerido:

```bash
git checkout -b feat/minha-melhoria
commit-pattern commit --all
git push origin feat/minha-melhoria
```

---

## 📄 Licença

Este projeto está licenciado sob a licença MIT.

---

## 👑 Créditos

<p align="center">
  <strong>Made with 💜 by Royal Software Engineering</strong>
</p>

<p align="center">
  🚀 👑 🦀 💻 💜
</p>
