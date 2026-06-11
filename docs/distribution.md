[⬅️ Voltar para o README principal](../README.md)

# 👥 Como distribuir para colegas

---

## 📌 Opções de distribuição

Existem três formas principais de distribuir o **Commit Pattern CLI** para colegas:

1. 📦 Distribuir o binário compilado
2. 🧰 Instalar sem `sudo`
3. 🦀 Cada pessoa compilar localmente

---

## 📦 Opção 1: distribuir o binário compilado

Na sua máquina, compile em modo release:

```bash
cargo build --release
```

Crie a pasta de distribuição:

```bash
mkdir -p dist
```

Copie o binário:

```bash
cp target/release/commit-pattern dist/
```

Compacte:

```bash
tar -czf dist/commit-pattern-linux-x86_64.tar.gz -C dist commit-pattern
```

Envie o arquivo:

```txt
dist/commit-pattern-linux-x86_64.tar.gz
```

---

## 🖥️ Instalar na máquina do colega

Na máquina do colega:

```bash
tar -xzf commit-pattern-linux-x86_64.tar.gz
```

Instalação global:

```bash
sudo install -m 755 commit-pattern /usr/local/bin/commit-pattern
```

Teste:

```bash
commit-pattern list-types
```

---

## 🧰 Opção 2: instalar sem sudo

Na máquina do colega:

```bash
mkdir -p ~/.local/bin
cp commit-pattern ~/.local/bin/commit-pattern
chmod +x ~/.local/bin/commit-pattern
```

Adicione ao `PATH` temporariamente:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Para deixar permanente no `bash`:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Para deixar permanente no `zsh`:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Teste:

```bash
commit-pattern --help
```

---

## 🦀 Opção 3: colegas compilam localmente

Cada colega pode instalar direto pelo código-fonte:

```bash
git clone <url-do-repositorio>
cd commit-creator
cargo install --path . --force
```

Depois configurar o shell:

```bash
commit-pattern setup-shell
```

E testar:

```bash
commit-pattern list-types
```

---

## 🧪 Validar binário

Depois de instalado, valide:

```bash
which commit-pattern
```

```bash
commit-pattern --version
```

```bash
commit-pattern --help
```

---

## 🧭 Recomendação para times

Para times pequenos usando Linux, a opção mais simples é:

```bash
cargo build --release
mkdir -p dist
cp target/release/commit-pattern dist/
tar -czf dist/commit-pattern-linux-x86_64.tar.gz -C dist commit-pattern
```

Depois cada pessoa instala em `/usr/local/bin` ou `~/.local/bin`.

---

[⬅️ Voltar para o README principal](../README.md)
