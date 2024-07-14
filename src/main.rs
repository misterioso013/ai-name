
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    should_use_specific_language: bool,
    idioma: String,
    descricao: String,
}
#[tokio::main]
async fn main() {
    println!("{}", "=========================================================".green());
    println!("{}", "                  AI Name Generator".blue().bold());
    println!("{}", "=========================================================".green());

    match std::env::var("GOOGLE_AI_API_KEY") {
        Ok(_) => println!("{}", "Chave de API Google AI encontrada!".yellow()),
        Err(_) => {
            println!("{}{}{}{}", "Chave de API Google AI não encontrada! ".red(), "Por favor, defina a variável de ambiente ".red().bold(), "GOOGLE_AI_API_KEY ".red().bold().underline(), "com a sua chave de API OpenAI".red());
            println!("{}{}","Consulte a documentação em ".white(), "https://github.com/misterioso013/ai-name".white().bold().underline());
            return;
        }
    }

    // pergunte ao usuário se ele deseja se ele deseja que o nome seja em um idioma específico (ex: en, pt, es) ou der enter para qualquer idioma, depois pergunte uma descrição do que ele deseja criar
    let should_use_specific_language: bool = Confirm::new()
        .with_prompt("Deseja que o nome seja em um idioma específico?")
        .interact()
        .unwrap();

    let idioma: String = if should_use_specific_language {
        Input::new()
            .with_prompt("Digite o idioma desejado (ex: english, português, español)")
            .interact()
            .unwrap()
    } else {
        String::from("not specified")
    };

    let descricao: String = Input::new()
        .with_prompt("Digite uma descrição do que deseja criar")
        .interact()
        .unwrap();

    println!("Idioma específico: {}", idioma);
    println!("Descrição: {}", descricao);

    generate_name(should_use_specific_language, idioma, descricao);
}

async fn api_request(specific_language: bool, language: String, description: String) {

    let context: String = format!("cite examples of possible names in the {} language to be used based on the description, you must respond in json format following this pattern: { \"name1\", \"name2\"...}".to_string(), language);
    let api_key = std::env::var("GOOGLE_AI_API_KEY").unwrap();
    
}

fn generate_name(should_use_specific_language: bool, idioma: String, descricao: String) {
    let config = Config {
        should_use_specific_language,
        idioma,
        descricao,
    };
    let name_list = vec!["João", "Maria", "José", "Ana"];
    
    // exibir os nomes disponíveis para o usuário escolher
    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha um nome")
        .items(&name_list)
        .default(0)
        .interact()
        .unwrap();

    println!("Nome gerado: {}", name_list[index]);
}
