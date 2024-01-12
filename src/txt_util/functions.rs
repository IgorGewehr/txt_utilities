// excel_processing.rs

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Função para extrair logradouro
pub fn extract_logradouro(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path)?;

    for line in reader.lines() {
        let line = line?;

        // Verifica se a linha após remoção de espaços em branco está vazia
        let cleaned_line = line.trim();
        if !cleaned_line.is_empty() {
            // Escreve a linha no arquivo de saída se não for uma linha em branco
            writeln!(output_file, "{}", cleaned_line)?;
        }
    }

    Ok(())
}

// Função para extrair números
pub fn extract_numbers(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path)?;

    for line in reader.lines() {
        let line = line?;
        if let Some(index) = line.find(',') {
            let rest_of_line = &line[index + 1..].trim();
            writeln!(output_file, "{}", rest_of_line)?;
        }
    }

    Ok(())
}

// Função para remover linhas em branco e espaços
pub fn remove_blank_lines(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path)?;

    for line in reader.lines() {
        let line = line?;

        // Remove espaços em branco da linha
        let cleaned_line = line.trim();

        // Verifica se a linha após remoção de espaços em branco está vazia
        if !cleaned_line.is_empty() {
            // Escreve a linha no arquivo de saída se não for uma linha em branco
            writeln!(output_file, "{}", cleaned_line)?;
        }
    }

    Ok(())
}
