[◀️ Página anterior: Pré-requisitos](prerequisites.md)  
[⬅️ Voltar para o README principal](../README.md)  

# 🦀 Instalando Rust

---

## 📌 Instalação oficial

A forma recomendada de instalar Rust é usando o `rustup`.

Execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Durante a instalação, escolha a opção padrão.

---

## 🔄 Carregar ambiente do Cargo

Depois da instalação, execute:

```bash
source "$HOME/.cargo/env"
```

Ou feche e abra o terminal.

---

## ✅ Validar instalação

Verifique o Rust:

```bash
rustc --version
```

Verifique o Cargo:

```bash
cargo --version
```

---

## 📦 Onde o Cargo instala binários?

Por padrão, binários instalados com `cargo install` ficam em:

```txt
~/.cargo/bin
```

O **Commit Pattern CLI** possui um comando para configurar esse caminho automaticamente no shell:

```bash
commit-pattern setup-shell
```

---

## 🧰 Shells suportados

O comando `setup-shell` suporta:

| Shell  | Arquivo configurado          |
| ------ | ---------------------------- |
| `bash` | `~/.bashrc`                  |
| `zsh`  | `~/.zshrc`                   |
| `fish` | `~/.config/fish/config.fish` |

---

## 🧪 Teste final

```bash
rustc --version
cargo --version
```

Se ambos os comandos funcionarem, você já pode compilar o projeto.

---

## 🚀 Próximo passo

Depois de instalar Rust, veja como distribuir a aplicação:

[➡️ Próxima página: 👥 Como distribuir para colegas](distribution.md)

---

[◀️ Página anterior: Pré-requisitos](prerequisites.md)  
[⬅️ Voltar para o README principal](../README.md)  