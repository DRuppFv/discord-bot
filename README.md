# Bot do discord oficial da codify community.

# Como eu posso rodar ele?

## Requisitos
- [Compilador rust](https://www.rust-lang.org/)
- [Um token do discord](https://discord.com/developers/applications)
- Conexão com a internet.

## Compilando
Depois de instalar o rust, execute o comando: `cargo build --release`

## Arquivo de configuração
Aviso: Caso você ignore essa parte, o bot irá criar o arquivo automaticamente para você, e você pode configurar como quiser

Exemplo de um `.env`:
```sh
DISCORD_TOKEN=SeuTokenDoDiscord
CODIFY_GUILD_ID=0000000000
```

## Finalmente executando ele
É somente executar: `cargo run --release` que o Rust dará conta do caso.


