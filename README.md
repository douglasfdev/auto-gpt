# Projeto para Aprender e Entender Rust

Para iniciar o projeto temos que executar o comando **`docker-compose up`** para subir o container, e para buildar novamente o projeto temos que rodar **`docker-compose build`** pois cada alteração no código ele não tem um runtime onde capte em nível de auto reload a instancia da aplicação.
Então será necessário toda vez que tiver uma alteração rodar o comando docker-compose build, ou simplesmente instalar o Rust em sua máquina, rodar o comando **`cargo update`**, **`cargo build`** e **`cargo run`**.

Para o Rust fazer a documentação de seu código basta rodar o comando **`cargo doc`** e para visualizar basta rodar o comando **`cargo doc --open`**, e escolher seu navegador favorito para visualizar a documentação de seu programa.

Para executar a documentação da suposta biblioteca, basta rodar o comando **`cd add`** e depois **`cargo doc --open`**