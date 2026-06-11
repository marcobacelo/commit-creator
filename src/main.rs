use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const COMMIT_TYPES: &[CommitType] = &[
    CommitType {
        name: "feat",
        emoji: "✨",
        description: "Nova funcionalidade",
    },
    CommitType {
        name: "fix",
        emoji: "🐛",
        description: "Correção de bug",
    },
    CommitType {
        name: "docs",
        emoji: "📝",
        description: "Documentação",
    },
    CommitType {
        name: "test",
        emoji: "✅",
        description: "Testes",
    },
    CommitType {
        name: "build",
        emoji: "📦",
        description: "Build, dependências ou empacotamento",
    },
    CommitType {
        name: "perf",
        emoji: "⚡",
        description: "Melhoria de performance",
    },
    CommitType {
        name: "style",
        emoji: "💄",
        description: "Formatação, estilo ou lint",
    },
    CommitType {
        name: "refactor",
        emoji: "♻️",
        description: "Refatoração sem alterar comportamento",
    },
    CommitType {
        name: "chore",
        emoji: "🔧",
        description: "Tarefas auxiliares",
    },
    CommitType {
        name: "ci",
        emoji: "👷",
        description: "Integração contínua",
    },
    CommitType {
        name: "raw",
        emoji: "🗃️",
        description: "Arquivos raw, configs ou dados",
    },
    CommitType {
        name: "cleanup",
        emoji: "🧹",
        description: "Limpeza de código",
    },
    CommitType {
        name: "remove",
        emoji: "🔥",
        description: "Remoção de código, arquivo ou dependência",
    },
    CommitType {
        name: "init",
        emoji: "🎉",
        description: "Commit inicial",
    },
];

#[derive(Clone)]
struct CommitType {
    name: &'static str,
    emoji: &'static str,
    description: &'static str,
}

#[derive(Parser)]
#[command(
    name = "commit-pattern",
    version,
    about = "CLI interativo para criar commits padronizados",
    long_about = "CLI interativo para criar commits no padrão Conventional Commits com emojis opcionais.\n\nExemplos:\n  commit-pattern commit\n  commit-pattern commit --all\n  commit-pattern validate --message \"feat(auth): adicionar login\"\n  commit-pattern install-hook --shared"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Abre um assistente interativo e executa git commit
    Commit {
        /// Executa git add . antes do commit
        #[arg(short, long)]
        all: bool,

        /// Apenas mostra a mensagem gerada, sem commitar
        #[arg(long)]
        dry_run: bool,

        /// Pula a confirmação final
        #[arg(short, long)]
        yes: bool,
    },

    /// Valida uma mensagem de commit
    Validate {
        /// Mensagem passada diretamente pela CLI
        #[arg(short, long, conflicts_with = "file")]
        message: Option<String>,

        /// Arquivo de mensagem recebido pelo hook commit-msg
        #[arg(short, long, conflicts_with = "message")]
        file: Option<PathBuf>,

        /// Exige no máximo 4 palavras na descrição da primeira linha
        #[arg(long)]
        max_4_words: bool,

        /// Tamanho máximo da primeira linha
        #[arg(long, default_value_t = 72)]
        max_subject_len: usize,
    },

    /// Lista os tipos aceitos
    ListTypes,

    /// Instala hook commit-msg para validação
    InstallHook {
        /// Instala em .githooks e configura core.hooksPath, melhor para versionar no repositório
        #[arg(long)]
        shared: bool,
    },

    /// Adiciona ~/.cargo/bin ao PATH do shell atual
    SetupShell,

    /// Mostra instruções de build e distribuição
    BuildHelp,
}

#[derive(Debug)]
struct CommitAnswers {
    commit_type: String,
    emoji: Option<String>,
    scope: Option<String>,
    description: String,
    breaking_change: bool,
    body: Option<String>,
    footer: Option<String>,
}

struct ParsedHeader {
    commit_type: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { all, dry_run, yes } => run_interactive_commit(all, dry_run, yes),

        Commands::Validate {
            message,
            file,
            max_4_words,
            max_subject_len,
        } => {
            let raw_message = read_message(message, file)?;
            validate_commit_message(&raw_message, max_4_words, max_subject_len)?;

            println!("{}", "Commit message válida.".green());
            Ok(())
        }

        Commands::ListTypes => {
            print_commit_types();
            Ok(())
        }

        Commands::InstallHook { shared } => install_hook(shared),

        Commands::SetupShell => setup_shell(),

        Commands::BuildHelp => {
            print_build_help();
            Ok(())
        }
    }
}

fn run_interactive_commit(all: bool, dry_run: bool, yes: bool) -> Result<()> {
    ensure_git_repository()?;

    if all {
        println!("{}", "Executando git add .".cyan());
        run_git(&["add", "."])?;
    }

    ensure_has_staged_changes()?;

    let answers = ask_commit_questions()?;
    let message = build_commit_message(&answers);

    validate_commit_message(&message, false, 72)?;

    println!();
    println!("{}", "Mensagem gerada:".bold());
    println!("{}", message.green());
    println!();

    if dry_run {
        println!("{}", "Dry run ativo. Nenhum commit foi criado.".yellow());
        return Ok(());
    }

    if !yes {
        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Confirmar commit?")
            .default(true)
            .interact()?;

        if !confirmed {
            println!("{}", "Commit cancelado.".yellow());
            return Ok(());
        }
    }

    run_git_commit(&message)?;

    println!("{}", "Commit criado com sucesso.".green());

    Ok(())
}

fn ask_commit_questions() -> Result<CommitAnswers> {
    let theme = ColorfulTheme::default();

    let items: Vec<String> = COMMIT_TYPES
        .iter()
        .map(|item| format!("{} {:<10} {}", item.emoji, item.name, item.description))
        .collect();

    let selected = Select::with_theme(&theme)
        .with_prompt("Qual é o tipo do commit?")
        .items(&items)
        .default(0)
        .interact()?;

    let selected_type = &COMMIT_TYPES[selected];

    let use_emoji = Confirm::with_theme(&theme)
        .with_prompt("Deseja usar emoji na mensagem?")
        .default(true)
        .interact()?;

    let scope_input: String = Input::with_theme(&theme)
        .with_prompt("Escopo, opcional. Ex: auth, api, infra")
        .allow_empty(true)
        .interact_text()?;

    let description: String = Input::with_theme(&theme)
        .with_prompt("Descrição curta")
        .validate_with(|input: &String| -> std::result::Result<(), &str> {
            if input.trim().is_empty() {
                Err("A descrição não pode ser vazia")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let breaking_change = Confirm::with_theme(&theme)
        .with_prompt("É uma breaking change?")
        .default(false)
        .interact()?;

    let body_input: String = Input::with_theme(&theme)
        .with_prompt("Corpo do commit, opcional")
        .allow_empty(true)
        .interact_text()?;

    let footer_input: String = Input::with_theme(&theme)
        .with_prompt("Footer, opcional. Ex: Closes #123")
        .allow_empty(true)
        .interact_text()?;

    Ok(CommitAnswers {
        commit_type: selected_type.name.to_string(),
        emoji: if use_emoji {
            Some(selected_type.emoji.to_string())
        } else {
            None
        },
        scope: empty_to_none(scope_input),
        description: description.trim().to_string(),
        breaking_change,
        body: empty_to_none(body_input),
        footer: empty_to_none(footer_input),
    })
}

fn build_commit_message(answers: &CommitAnswers) -> String {
    let mut header = String::new();

    if let Some(emoji) = &answers.emoji {
        header.push_str(emoji);
        header.push(' ');
    }

    header.push_str(&answers.commit_type);

    if let Some(scope) = &answers.scope {
        header.push('(');
        header.push_str(scope.trim());
        header.push(')');
    }

    if answers.breaking_change {
        header.push('!');
    }

    header.push_str(": ");
    header.push_str(answers.description.trim());

    let mut message = header;

    if let Some(body) = &answers.body {
        message.push_str("\n\n");
        message.push_str(body.trim());
    }

    if answers.breaking_change {
        message.push_str("\n\n");
        message.push_str("BREAKING CHANGE: ");
        message.push_str(answers.description.trim());
    }

    if let Some(footer) = &answers.footer {
        message.push_str("\n\n");
        message.push_str(footer.trim());
    }

    message
}

fn empty_to_none(value: String) -> Option<String> {
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn ensure_git_repository() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("falha ao verificar repositório Git")?;

    if !output.status.success() {
        bail!("este diretório não parece ser um repositório Git");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim() != "true" {
        bail!("este diretório não parece ser um repositório Git");
    }

    Ok(())
}

fn ensure_has_staged_changes() -> Result<()> {
    let status = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .context("falha ao verificar arquivos staged")?;

    if status.success() {
        bail!(
            "não há alterações staged para commitar. Use git add antes ou execute: commit-pattern commit --all"
        );
    }

    Ok(())
}

fn run_git_commit(message: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("falha ao executar git commit")?;

    if !status.success() {
        bail!("git commit falhou");
    }

    Ok(())
}

fn read_message(message: Option<String>, file: Option<PathBuf>) -> Result<String> {
    match (message, file) {
        (Some(message), None) => Ok(message),
        (None, Some(file)) => fs::read_to_string(&file)
            .with_context(|| format!("não foi possível ler o arquivo {}", file.display())),
        _ => bail!("use --message ou --file"),
    }
}

fn validate_commit_message(
    raw_message: &str,
    max_4_words: bool,
    max_subject_len: usize,
) -> Result<()> {
    let subject = first_non_comment_line(raw_message)
        .context("mensagem de commit vazia")?
        .trim();

    if should_ignore_commit(subject) {
        return Ok(());
    }

    if subject.len() > max_subject_len {
        bail!(
            "primeira linha muito longa: {} caracteres. Máximo permitido: {}.",
            subject.len(),
            max_subject_len
        );
    }

    let normalized = strip_optional_emoji(subject);

    let (header, description) = normalized.split_once(": ").ok_or_else(|| {
        anyhow::anyhow!(
            "formato inválido. Use: tipo: descrição, tipo(escopo): descrição ou emoji tipo(escopo): descrição"
        )
    })?;

    if description.trim().is_empty() {
        bail!("descrição não pode ser vazia");
    }

    if max_4_words && description.split_whitespace().count() > 4 {
        bail!("descrição deve ter no máximo 4 palavras");
    }

    let parsed_header = parse_header(header)?;

    let allowed_types: Vec<&str> = COMMIT_TYPES.iter().map(|item| item.name).collect();

    if !allowed_types.contains(&parsed_header.commit_type.as_str()) {
        bail!(
            "tipo '{}' não é aceito. Tipos aceitos: {}",
            parsed_header.commit_type,
            allowed_types.join(", ")
        );
    }

    Ok(())
}

fn first_non_comment_line(raw_message: &str) -> Option<&str> {
    raw_message
        .lines()
        .find(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
}

fn should_ignore_commit(subject: &str) -> bool {
    subject.starts_with("Merge ")
        || subject.starts_with("Revert ")
        || subject.starts_with("fixup! ")
        || subject.starts_with("squash! ")
}

fn strip_optional_emoji(subject: &str) -> &str {
    let mut parts = subject.splitn(2, char::is_whitespace);
    let first_token = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or(subject);

    if is_emoji_shortcode(first_token) || is_unicode_emoji_token(first_token) {
        rest.trim_start()
    } else {
        subject
    }
}

fn is_emoji_shortcode(token: &str) -> bool {
    token.starts_with(':')
        && token.ends_with(':')
        && token.len() > 2
        && token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == ':' || c == '_' || c == '-' || c == '+')
}

fn is_unicode_emoji_token(token: &str) -> bool {
    token.chars().any(|c| !c.is_ascii())
}

fn parse_header(header: &str) -> Result<ParsedHeader> {
    if header.contains(' ') {
        bail!("cabeçalho não pode conter espaços antes dos dois pontos");
    }

    let header = header.strip_suffix('!').unwrap_or(header);

    let commit_type = if let Some(scope_start) = header.find('(') {
        let scope_end = header
            .find(')')
            .ok_or_else(|| anyhow::anyhow!("escopo inválido. Use tipo(escopo): descrição"))?;

        if scope_end != header.len() - 1 {
            bail!("conteúdo inválido após o escopo");
        }

        let commit_type = &header[..scope_start];
        let scope = &header[scope_start + 1..scope_end];

        if scope.trim().is_empty() {
            bail!("escopo não pode ser vazio");
        }

        commit_type
    } else {
        header
    };

    if commit_type.is_empty() {
        bail!("tipo do commit não pode ser vazio");
    }

    if !commit_type
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-')
    {
        bail!("tipo deve estar em minúsculas, exemplo: feat, fix, docs");
    }

    Ok(ParsedHeader {
        commit_type: commit_type.to_string(),
    })
}

fn install_hook(shared: bool) -> Result<()> {
    ensure_git_repository()?;

    if shared {
        install_shared_hook()
    } else {
        install_local_hook()
    }
}

fn install_shared_hook() -> Result<()> {
    let hooks_dir = PathBuf::from(".githooks");
    fs::create_dir_all(&hooks_dir)?;

    let hook_path = hooks_dir.join("commit-msg");
    write_hook_script(&hook_path)?;

    run_git(&["config", "core.hooksPath", ".githooks"])?;

    println!(
        "{}",
        "Hook instalado em .githooks/commit-msg e core.hooksPath configurado.".green()
    );

    println!();
    println!("{}", "Agora versionar o hook é recomendado:".bold());
    println!("  git add .githooks");
    println!("  git commit -m \"chore: adicionar hook de commit\"");

    Ok(())
}

fn install_local_hook() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks"])
        .output()
        .context("falha ao localizar diretório de hooks do Git")?;

    if !output.status.success() {
        bail!("este diretório não parece ser um repositório Git");
    }

    let hooks_dir = String::from_utf8(output.stdout)?.trim().to_string();
    let hook_path = PathBuf::from(hooks_dir).join("commit-msg");

    write_hook_script(&hook_path)?;

    println!("{} {}", "Hook instalado em".green(), hook_path.display());

    Ok(())
}

fn write_hook_script(path: &Path) -> Result<()> {
    let script = r#"#!/bin/sh
commit-pattern validate --file "$1"
"#;

    fs::write(path, script)
        .with_context(|| format!("não foi possível escrever {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }

    Ok(())
}

fn setup_shell() -> Result<()> {
    let home = env::var("HOME").context("não foi possível identificar a variável HOME")?;

    let cargo_bin = format!("{}/.cargo/bin", home);
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("zsh") {
        let shell_file = PathBuf::from(format!("{}/.zshrc", home));

        append_path_to_file(
            &shell_file,
            &format!(r#"export PATH="{}:$PATH""#, cargo_bin),
            "commit-pattern",
        )?;

        println!("{}", "PATH configurado no ~/.zshrc".green());
        println!("{}", "Execute para aplicar no terminal atual:".bold());
        println!("  source ~/.zshrc");
    } else if shell.contains("bash") {
        let shell_file = PathBuf::from(format!("{}/.bashrc", home));

        append_path_to_file(
            &shell_file,
            &format!(r#"export PATH="{}:$PATH""#, cargo_bin),
            "commit-pattern",
        )?;

        println!("{}", "PATH configurado no ~/.bashrc".green());
        println!("{}", "Execute para aplicar no terminal atual:".bold());
        println!("  source ~/.bashrc");
    } else if shell.contains("fish") {
        let fish_config_dir = PathBuf::from(format!("{}/.config/fish", home));
        fs::create_dir_all(&fish_config_dir)?;

        let shell_file = fish_config_dir.join("config.fish");

        append_path_to_file(
            &shell_file,
            &format!("set -gx PATH {} $PATH", cargo_bin),
            "commit-pattern",
        )?;

        println!(
            "{}",
            "PATH configurado no ~/.config/fish/config.fish".green()
        );
        println!("{}", "Execute para aplicar no terminal atual:".bold());
        println!("  source ~/.config/fish/config.fish");
    } else {
        println!("{}", "Shell não identificado automaticamente.".yellow());
        println!("Adicione manualmente ao arquivo de configuração do seu shell:");
        println!();
        println!(r#"export PATH="{}:$PATH""#, cargo_bin);
    }

    println!();
    println!(
        "{}",
        "Observação: nenhum programa consegue alterar diretamente o ambiente do terminal pai já aberto.".yellow()
    );
    println!("Por isso é necessário rodar source ou abrir um novo terminal.");

    Ok(())
}

fn append_path_to_file(path: &Path, line: &str, marker: &str) -> Result<()> {
    let current_content = fs::read_to_string(path).unwrap_or_default();

    if current_content.contains(line) {
        println!("{}", "Configuração já existia. Nada foi alterado.".yellow());
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("não foi possível abrir {}", path.display()))?;

    writeln!(file)?;
    writeln!(file, "# {}", marker)?;
    writeln!(file, "{}", line)?;

    Ok(())
}

fn run_git(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .status()
        .context("falha ao executar git")?;

    if !status.success() {
        bail!("comando git falhou: git {}", args.join(" "));
    }

    Ok(())
}

fn print_commit_types() {
    println!("{}", "Tipos aceitos:".bold());

    for commit_type in COMMIT_TYPES {
        println!(
            "  {} {:<10} {}",
            commit_type.emoji, commit_type.name, commit_type.description
        );
    }
}

fn print_build_help() {
    println!("{}", "Comandos de build".bold());
    println!();

    println!("{}", "Build de desenvolvimento:".cyan());
    println!("  cargo build");
    println!();

    println!("{}", "Executar localmente durante desenvolvimento:".cyan());
    println!("  cargo run -- commit --dry-run");
    println!("  cargo run -- list-types");
    println!();

    println!("{}", "Build otimizado para distribuição:".cyan());
    println!("  cargo build --release");
    println!();

    println!("{}", "Binário gerado:".cyan());
    println!("  target/release/commit-pattern");
    println!();

    println!("{}", "Instalar localmente com Cargo:".cyan());
    println!("  cargo install --path . --force");
    println!();

    println!("{}", "Adicionar ~/.cargo/bin ao PATH:".cyan());
    println!("  commit-pattern setup-shell");
    println!();

    println!("{}", "Empacotar para colegas Linux x86_64:".cyan());
    println!("  mkdir -p dist");
    println!("  cp target/release/commit-pattern dist/");
    println!("  tar -czf dist/commit-pattern-linux-x86_64.tar.gz -C dist commit-pattern");
    println!();

    println!("{}", "Instalar em outra máquina com sudo:".cyan());
    println!("  tar -xzf commit-pattern-linux-x86_64.tar.gz");
    println!("  sudo install -m 755 commit-pattern /usr/local/bin/commit-pattern");
    println!();

    println!("{}", "Instalar em outra máquina sem sudo:".cyan());
    println!("  mkdir -p ~/.local/bin");
    println!("  cp commit-pattern ~/.local/bin/commit-pattern");
    println!("  chmod +x ~/.local/bin/commit-pattern");
    println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_basic_commit() {
        assert!(validate_commit_message("feat: adicionar login", false, 72).is_ok());
    }

    #[test]
    fn accepts_scope() {
        assert!(validate_commit_message("fix(api): corrigir timeout", false, 72).is_ok());
    }

    #[test]
    fn accepts_shortcode_emoji() {
        assert!(validate_commit_message(":bug: fix(api): corrigir timeout", false, 72).is_ok());
    }

    #[test]
    fn accepts_unicode_emoji() {
        assert!(validate_commit_message("✨ feat(auth): adicionar login", false, 72).is_ok());
    }

    #[test]
    fn accepts_breaking_change() {
        assert!(validate_commit_message("feat(api)!: alterar contrato", false, 72).is_ok());
    }

    #[test]
    fn rejects_invalid_type() {
        assert!(validate_commit_message("feature: adicionar login", false, 72).is_err());
    }

    #[test]
    fn rejects_missing_description() {
        assert!(validate_commit_message("feat:", false, 72).is_err());
    }

    #[test]
    fn rejects_empty_scope() {
        assert!(validate_commit_message("feat(): adicionar login", false, 72).is_err());
    }
}