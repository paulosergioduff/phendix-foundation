#!/bin/bash

# Verifica se foi fornecido o nome do projeto como argumento
if [ -z "$1" ]; then
    echo "Erro: É necessário fornecer o nome do projeto como argumento."
    exit 1
fi

# Nome do projeto fornecido como argumento
nome_do_projeto="$1"

# Verifica se cada arquivo de prompt tem um equivalente na pasta doc
for prompt_file in "./projects/$nome_do_projeto/auto/prompt/"*/*.txt; do
    doc_file="./projects/$nome_do_projeto/doc/$(basename "$prompt_file" .txt).doc"
    if [ ! -f "$doc_file" ]; then
        echo -e "\e[91mError:\e[0m Missing documentation file for $prompt_file"
        exit 1
    fi
done

# Verifica se o tamanho do arquivo na pasta doc é maior ou igual ao tamanho do arquivo na pasta prompt
for prompt_file in "./projects/$nome_do_projeto/auto/prompt/"*/*.txt; do
    doc_file="./projects/$nome_do_projeto/doc/$(basename "$prompt_file" .txt).doc"
    if [ -f "$doc_file" ]; then
        prompt_size=$(wc -c < "$prompt_file")
        doc_size=$(wc -c < "$doc_file")
        if [ "$doc_size" -lt "$prompt_size" ]; then
            echo -e "\e[91mError:\e[0m Documentation file $doc_file size is less than prompt file $prompt_file size"
            exit 1
        fi
    fi
done

echo -e "\e[32mAll business rules passed successfully!\e[0m"
