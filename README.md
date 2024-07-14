# AI Name
Executável criado para dar nomes a projetos usando Inteligência Artificial.

## Requirements
- [Rust](https://www.rust-lang.org/learn/get-started)
- [API key do Google AI](https://aistudio.google.com/app/apikey)
- Variáveis de ambiente:
  - `GOOGLE_AI_API_KEY`: chave de acesso à API do Google AI

## Installation
Para instalar o projeto, basta baixar o binário da última versão disponível no [repositório de releases](https://github.com/misterioso013/ai-name/releases).


## Development
Projeto desenvolvido em Rust, visando a simplicidade e eficiência.

## Usage
Para usar o executável em desenvolvimento, basta rodar o comando:
```bash
# usando cargo watch
cargo watch -x run
# usando nodemon
npx nodemon --exec "cargo run" ./src/main.rs
```