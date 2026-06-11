fn print_manual() {
    println!("{}", "commit-pattern - Manual completo".bold());
    println!();

    println!("{}", "DESCRIÇÃO".cyan().bold());
    println!("  CLI interativo para criar commits padronizados usando Conventional Commits com emojis opcionais.");
    println!("  A aplicação pode criar commits, validar mensagens, instalar hooks e auxiliar na distribuição do binário.");
    println!();

    println!("{}", "COMANDOS DISPONÍVEIS".cyan().bold());
    println!();

    println!("{}", "  commit-pattern commit".green().bold());
    println!("    Abre o assistente interativo e executa git commit.");
    println!("    Usa somente arquivos que já estão staged.");
    println!();
    println!("    Exemplo:");
    println!("      git add src/main.rs");
    println!("      commit-pattern commit");
    println!();

    println!("{}", "  commit-pattern commit --all".green().bold());
    println!("    Executa git add . antes de abrir o assistente.");
    println!("    Importante: usa git add ., não git add -A.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern commit --all");
    println!();

    println!("{}", "  commit-pattern commit --dry-run".green().bold());
    println!("    Gera a mensagem, mas não executa git commit.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern commit --dry-run");
    println!();

    println!("{}", "  commit-pattern commit --yes".green().bold());
    println!("    Pula a confirmação final.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern commit --yes");
    println!();

    println!("{}", "  commit-pattern validate --message".green().bold());
    println!("    Valida uma mensagem informada diretamente pela CLI.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern validate --message \"feat(auth): adicionar login\"");
    println!();

    println!("{}", "  commit-pattern validate --file".green().bold());
    println!("    Valida uma mensagem a partir de um arquivo.");
    println!("    Usado principalmente pelo hook commit-msg.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern validate --file .git/COMMIT_EDITMSG");
    println!();

    println!("{}", "  commit-pattern list-types".green().bold());
    println!("    Lista todos os tipos de commit aceitos.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern list-types");
    println!();

    println!("{}", "  commit-pattern install-hook".green().bold());
    println!("    Instala hook local em .git/hooks/commit-msg.");
    println!("    Esse hook não é versionado.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern install-hook");
    println!();

    println!("{}", "  commit-pattern install-hook --shared".green().bold());
    println!("    Instala hook compartilhado em .githooks/commit-msg.");
    println!("    Também executa git config core.hooksPath .githooks.");
    println!("    Essa é a opção recomendada para times.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern install-hook --shared");
    println!();

    println!("{}", "  commit-pattern setup-shell".green().bold());
    println!("    Adiciona ~/.cargo/bin ao PATH do shell detectado.");
    println!("    Suporta bash, zsh e fish.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern setup-shell");
    println!();

    println!("{}", "  commit-pattern build-help".green().bold());
    println!("    Mostra instruções de build, instalação e distribuição.");
    println!();
    println!("    Exemplo:");
    println!("      commit-pattern build-help");
    println!();

    println!("{}", "  commit-pattern manual".green().bold());
    println!("    Mostra este manual completo.");
    println!();

    println!("{}", "PADRÃO DE COMMIT".cyan().bold());
    println!("  Formato:");
    println!("    emoji tipo(escopo): descrição");
    println!();
    println!("  Exemplos:");
    println!("    ✨ feat(auth): adicionar login social");
    println!("    🐛 fix(api): corrigir timeout");
    println!("    📝 docs: atualizar README");
    println!("    ♻️ refactor(service): simplificar validação");
    println!("    🔥 remove: remover código obsoleto");
    println!();

    println!("{}", "TIPOS ACEITOS".cyan().bold());
    for commit_type in COMMIT_TYPES {
        println!(
            "  {} {:<10} {}",
            commit_type.emoji, commit_type.name, commit_type.description
        );
    }
    println!();

    println!("{}", "FLUXO RECOMENDADO".cyan().bold());
    println!("  Para controle fino dos arquivos:");
    println!("    git add arquivo-especifico");
    println!("    commit-pattern commit");
    println!();
    println!("  Para adicionar tudo do diretório atual:");
    println!("    commit-pattern commit --all");
    println!();

    println!("{}", "HOOK RECOMENDADO".cyan().bold());
    println!("  O hook recomendado é commit-msg.");
    println!("  Ele valida a mensagem final do commit e impede mensagens fora do padrão.");
    println!();
    println!("  Instalação recomendada:");
    println!("    commit-pattern install-hook --shared");
    println!();

    println!("{}", "BUILD".cyan().bold());
    println!("  Desenvolvimento:");
    println!("    cargo build");
    println!();
    println!("  Release:");
    println!("    cargo build --release");
    println!();
    println!("  Binário:");
    println!("    target/release/commit-pattern");
    println!();

    println!("{}", "CRÉDITOS".cyan().bold());
    println!("  Made with <3 by Royal Software Engineering.");
}