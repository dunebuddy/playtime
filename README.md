# playtime

`playtime` é um pequeno utilitário em Rust para executar jogos fora da Steam e registrar automaticamente o tempo de jogo de cada sessão.

A ideia principal é permitir rodar jogos por meio de wrappers personalizados, Proton, Wine, scripts, launchers locais ou qualquer outro comando, salvando um histórico simples e consultável de sessões.

## Objetivo

Criar um binário de linha de comando chamado `playtime` capaz de:

1. Ler configurações de jogos a partir de arquivos.
2. Executar um comando associado a um jogo.
3. Registrar o início e o fim de cada sessão.
4. Salvar metadados suficientes para calcular o tempo total jogado.
5. Permitir uso rápido também sem arquivo de configuração, passando nome do jogo e comando diretamente pela CLI.

## Linguagem

O projeto deve ser implementado em **Rust**.

## Formato de configuração

O programa deve aceitar arquivos de configuração em um formato simples e amigável.

Formatos aceitáveis, em ordem de preferência:

1. **TOML** — recomendado para configuração local.
2. YAML — aceitável.
3. JSON — aceitável.

Para este projeto, TOML é preferível por ser mais focado em arquivos de configuração, mais simples de editar manualmente e bem suportado no ecossistema Rust.

Exemplo recomendado em TOML:

```toml
display_name = "Ghost of Tsushima Director's Cut"

command = [
  "/games/ghost/run.sh"
]
```

Exemplo com comando mais completo:

```toml
display_name = "Ghost of Tsushima Director's Cut"

command = [
  "env",
  "STEAM_COMPAT_DATA_PATH=/games/prefixes/ghost",
  "/games/proton/GE-Proton/proton",
  "run",
  "/games/GhostOfTsushima/GhostOfTsushima.exe"
]
```

O campo `command` deve ser uma lista/array de strings, não uma string única. Isso evita problemas de escaping, aspas e espaços em nomes de arquivos.

## Modos de execução esperados

O programa deve suportar pelo menos dois modos principais.

### 1. Rodar usando arquivo de configuração

Exemplo:

```bash
playtime ghost.toml
```

Nesse modo, o programa deve:

1. Ler o arquivo `ghost.toml`.
2. Extrair o nome bonito do jogo em `display_name`.
3. Extrair o comando em `command`.
4. Executar o comando.
5. Registrar a sessão.

### 2. Rodar passando nome bonito e comando direto

Exemplo:

```bash
playtime "Ghost of Tsushima DC" -- /games/ghost/run.sh
```

Outro exemplo:

```bash
playtime "Ghost of Tsushima DC" -- env STEAM_COMPAT_DATA_PATH=/games/prefixes/ghost /games/proton/GE-Proton/proton run /games/GhostOfTsushima/GhostOfTsushima.exe
```

Nesse modo:

- Tudo antes de `--` representa o nome bonito do jogo.
- Tudo depois de `--` representa o comando a ser executado.
- O comando deve ser preservado e salvo no histórico da sessão.

## Interface CLI inicial

A primeira versão pode usar uma interface simples:

```bash
playtime <config-file>
```

ou:

```bash
playtime "<display-name>" -- <command> [args...]
```

ou:

```bash
playtime list [-w|--wide] [filter...]
```

ou:

```bash
playtime info <game-id-prefix>
```

ou:

```bash
playtime sessions <game-id-prefix> [-d|--desc] [-w|--wide]
```

ou:

```bash
playtime session <session-id-prefix>
```

ou:

```bash
playtime --version
```

Exemplos:

```bash
playtime ghost.toml
```

```bash
playtime "Ghost of Tsushima DC" -- /games/ghost/run.sh
```

```bash
playtime list
```

```bash
playtime list --wide
```

```bash
playtime list ghost -w
```

```bash
playtime list -w ghost of
```

```bash
playtime info 3d9ebc7789
```

```bash
playtime sessions 3d9ebc7789
```

```bash
playtime sessions 3d9ebc7789 -d -w
```

```bash
playtime session a1b2c3d4e5
```

```bash
playtime --version
```

Futuramente, o projeto pode evoluir com mais subcomandos:

```bash
playtime run ghost.toml
playtime run "Ghost of Tsushima DC" -- /games/ghost/run.sh
playtime total
playtime sessions "Ghost of Tsushima DC"
```

Mas a primeira versão não precisa implementar tudo.

## Dados que devem ser salvos

Cada sessão de jogo deve ser registrada com os seguintes campos:

- `id`: identificador único da sessão.
- `game_id`: SHA-256 em hexadecimal calculado a partir do nome bonito normalizado.
- `session_number`: número sequencial da sessão daquele jogo.
- `display_name`: nome bonito do jogo.
- `command`: comando executado para iniciar o jogo.
- `started_at`: data e hora de início da sessão.
- `ended_at`: data e hora de fim da sessão.
- `duration_seconds`: duração da sessão em segundos.
- `exit_code`: código de saída do processo, quando disponível.
- `created_at`: data e hora em que o registro foi criado.

Exemplo conceitual de sessão:

```json
{
  "id": "9bfebc38eacb37dba15cc95f92e5f11e32a9ef063af6438e0e4c927c0f7fd0f4",
  "game_id": "3d9ebc77893e4cd56833e2bbf905eb1b2d9f005632d458ce709e660041e5a9ac",
  "session_number": 12,
  "display_name": "Ghost of Tsushima Director's Cut",
  "command": [
    "/games/ghost/run.sh"
  ],
  "started_at": "2026-05-14T21:05:10-03:00",
  "ended_at": "2026-05-14T23:17:44-03:00",
  "duration_seconds": 7954,
  "exit_code": 0,
  "created_at": "2026-05-14T23:17:44-03:00"
}
```

## Armazenamento

A primeira versão pode usar um arquivo local simples.

Opções aceitáveis:

1. **SQLite** — melhor opção para consultas futuras.
2. JSONL — mais simples para primeira implementação.
3. JSON único — aceitável, mas menos robusto.

Recomendação: usar **SQLite** desde o início, porque o projeto naturalmente vai precisar responder perguntas como:

- Quantas horas joguei no total?
- Quantas sessões tive?
- Qual foi minha sessão mais longa?
- Quando joguei pela última vez?
- Quanto joguei por jogo?

Caminho sugerido do banco:

```text
~/.local/share/playtime/playtime.db
```

O programa deve criar o diretório automaticamente se ele não existir.

## Modelo de tabela sugerido

Tabela: `sessions`

Campos:

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    game_id TEXT NOT NULL,
    session_number INTEGER NOT NULL,
    display_name TEXT NOT NULL,
    command_json TEXT NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL,
    exit_code INTEGER,
    created_at TEXT NOT NULL
);
```

Observações:

- `command_json` deve salvar o array de comando serializado como JSON.
- `game_id` deve ser o SHA-256 completo do `display_name` normalizado, salvo em hexadecimal.
- `id` deve ser um SHA-256 próprio da sessão, salvo em hexadecimal, para permitir busca por prefixo curto.
- Comandos futuros de consulta podem aceitar qualquer prefixo do `game_id`, desde que esse prefixo identifique um único jogo.
- `session_number` deve ser sequencial por `game_id`.
- Datas devem ser salvas em formato ISO 8601.
- O fuso horário local deve ser preservado quando possível.

## Comportamento durante execução

Quando o usuário rodar um jogo, o programa deve:

1. Resolver a configuração.
2. Salvar o timestamp de início.
3. Executar o processo filho.
4. Aguardar o processo terminar.
5. Salvar o timestamp de fim.
6. Calcular a duração.
7. Inserir a sessão no banco.
8. Encerrar sem imprimir resumo de sucesso.

A primeira versão deve ser silenciosa em caso de sucesso. Ferramentas para consultar o SQLite,
listar sessões e calcular totais devem ficar para comandos futuros.

## Listagem de jogos

O comando:

```bash
playtime list
```

deve listar uma linha por jogo, com:

- prefixo de 10 caracteres do `game_id`.
- nome bonito.

O comando:

```bash
playtime list [filter...]
```

também pode receber um texto para filtrar jogos pelo nome bonito ou pelo comando usado
em qualquer sessão. O resultado continua mostrando jogos únicos, não sessões.

Exemplos:

```bash
playtime list ghost
```

```bash
playtime list ghost of
```

```bash
playtime list -w proton
```

```bash
playtime list ghost -w
```

O comando:

```bash
playtime list -w [filter...]
```

ou:

```bash
playtime list --wide [filter...]
```

deve mostrar:

- `game_id` completo.
- nome bonito.
- número de sessões.
- tempo total no jogo.
- data da última sessão.

A data da última sessão deve usar `ended_at`, porque o histórico registra sessões
concluídas. Assim, a data representa quando a última sessão registrada terminou.

## Informações de um jogo

O comando:

```bash
playtime info <game-id-prefix>
```

deve procurar jogos cujo `game_id` comece com o valor informado. A busca deve ser
case-insensitive e o prefixo deve conter apenas caracteres hexadecimais.

Se nenhum jogo for encontrado, o programa deve mostrar erro.

Se mais de um jogo for encontrado, o programa deve mostrar uma mensagem de ambiguidade
com o nome bonito e o `game_id` completo de cada jogo encontrado, para o usuário passar
mais caracteres.

Se exatamente um jogo for encontrado, o programa deve mostrar uma visão em formato de
formulário com:

- id completo.
- nome do jogo.
- número de sessões.
- tempo total de jogo.
- data da última sessão.
- último comando utilizado para rodar o jogo.

Em seguida, deve mostrar uma tabela com as últimas 10 sessões, da mais recente para a
mais antiga, e um rodapé avisando que a tabela mostra no máximo as últimas 10 sessões.

## Listagem de sessões

O comando:

```bash
playtime sessions <game-id-prefix>
```

deve procurar o jogo pelo começo do `game_id` e listar todas as sessões desse jogo.
Por padrão, as sessões devem aparecer em ordem crescente.

O comando aceita:

- `-d` ou `--desc` para listar da mais recente para a mais antiga.
- `-w` ou `--wide` para mostrar ids completos e comando completo.

Na visualização padrão, a tabela deve mostrar:

- prefixo de 10 caracteres do id da sessão.
- número da sessão.
- início.
- fim.
- duração.
- exit code.
- comando truncado quando for grande.

Na visualização `--wide`, a tabela deve mostrar o id completo da sessão e o comando completo.

## Informações de uma sessão

O comando:

```bash
playtime session <session-id-prefix>
```

deve procurar sessões cujo `id` comece com o valor informado. A busca deve ser
case-insensitive e o prefixo deve conter apenas caracteres hexadecimais.

Se nenhum registro for encontrado, o programa deve mostrar erro. Se mais de uma sessão
for encontrada, deve mostrar uma mensagem de ambiguidade com o id completo, o nome do jogo,
o número da sessão e a data de fim. Se exatamente uma sessão for encontrada, deve mostrar
uma visão em formato de formulário com todos os dados da sessão.

## Tratamento de erro

O programa deve lidar com pelo menos estes casos:

### Arquivo de configuração não existe

```text
Erro: arquivo de configuração não encontrado: ghost.toml
```

### Arquivo de configuração inválido

```text
Erro: não foi possível ler a configuração.
Verifique se o arquivo contém display_name e command.
```

### Comando vazio

```text
Erro: nenhum comando foi informado para executar o jogo.
```

### Processo falha ao iniciar

```text
Erro: não foi possível iniciar o jogo.
```

Mesmo se o processo retornar código de erro, a sessão deve ser salva, desde que o processo tenha iniciado corretamente.

## Dependências Rust sugeridas

Possíveis crates:

- `clap` para parsing da CLI.
- `serde` para serialização/deserialização.
- `toml` para ler arquivos TOML.
- `serde_json` para salvar o comando como JSON.
- `rusqlite` para SQLite.
- `chrono` ou `time` para timestamps.
- `uuid` para gerar IDs de sessão.
- `dirs` ou `directories` para localizar `~/.local/share`.

Sugestão inicial:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
rusqlite = { version = "0.31", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v7", "serde"] }
directories = "5"
```

As versões podem ser ajustadas conforme necessário.

## Escopo da primeira versão

A primeira versão deve implementar:

- Ler configuração TOML.
- Rodar comando vindo do TOML.
- Rodar comando vindo diretamente da CLI.
- Registrar sessão no SQLite.
- Listar jogos registrados com `playtime list`.
- Mostrar informações de um jogo com `playtime info <game-id-prefix>`.
- Listar sessões de um jogo com `playtime sessions <game-id-prefix>`.
- Mostrar informações de uma sessão com `playtime session <session-id-prefix>`.
- Encerrar sem imprimir resumo de sucesso.

Não precisa implementar na primeira versão:

- Interface gráfica.
- Edição de registros.
- Importação de dados da Steam.
- Integração com Heroic/Lutris.
- Detecção automática de jogos.
- Estatísticas avançadas.
- Sincronização em nuvem.

## Exemplos de uso

### Arquivo `ghost.toml`

```toml
display_name = "Ghost of Tsushima Director's Cut"

command = [
  "/games/ghost/run.sh"
]
```

Rodando:

```bash
playtime ghost.toml
```

### Sem arquivo

```bash
playtime "Ghost of Tsushima Director's Cut" -- /games/ghost/run.sh
```

### Com Proton diretamente

```bash
playtime "Ghost of Tsushima Director's Cut" -- env STEAM_COMPAT_DATA_PATH=/games/prefixes/ghost /games/proton/GE-Proton/proton run /games/GhostOfTsushima/GhostOfTsushima.exe
```

## Filosofia do projeto

O `playtime` deve ser simples, previsível e transparente.

Ele não deve tentar ser um launcher completo. Ele deve ser apenas um wrapper confiável para medir tempo de execução de jogos.

A ideia é que o usuário continue controlando como seus jogos rodam, enquanto o `playtime` apenas envolve esse comando, mede a duração e guarda o histórico.

## Nome do binário

O binário final deve se chamar:

```text
playtime
```

Depois de compilado em release, ele poderá ser copiado para:

```text
~/.local/bin/playtime
```

Exemplo:

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/playtime ~/.local/bin/
```
