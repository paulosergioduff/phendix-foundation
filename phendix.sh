#!/bin/bash

# Verifica se foi fornecido um nome de projeto como argumento
if [ -z "$1" ]; then
    echo "Erro: É necessário fornecer um nome de projeto como argumento."
    exit 1
fi

# Nome do projeto fornecido como argumento
nome_do_projeto="$1"

# Cria a estrutura de pastas dentro da pasta do projeto
mkdir -p "projects/$nome_do_projeto/manual" "projects/$nome_do_projeto/auto/prompt/GPT-3.5_v1" "projects/$nome_do_projeto/auto/prompt/GPT-4.0_v2" "projects/$nome_do_projeto/doc" "projects/$nome_do_projeto/framework"

# Cria arquivos de exemplo nas pastas Prompt
touch "projects/$nome_do_projeto/auto/prompt/GPT-3.5_v1/files_Prompt_1.txt" "projects/$nome_do_projeto/auto/prompt/GPT-3.5_v1/files_Prompt_2.txt" "projects/$nome_do_projeto/auto/prompt/GPT-4.0_v2/files_Prompt_1.txt" "projects/$nome_do_projeto/auto/prompt/GPT-4.0_v2/files_Prompt_2.txt"

# Cria arquivos de exemplo nas pastas doc
touch "projects/$nome_do_projeto/doc/files_Prompt_1.doc" "projects/$nome_do_projeto/doc/files_Prompt_2.doc"

# Copia o arquivo run.sh para a pasta do projeto
cp run.sh "projects/$nome_do_projeto/run.sh"

echo "Estrutura de pastas para o projeto 'projects/$nome_do_projeto' criada com sucesso!"
echo "run.sh copied to 'projects/$nome_do_projeto' successfully!"
