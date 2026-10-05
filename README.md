# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

## Sobre o projeto

O ConectaStore é um sistema de recomendação desenvolvido para a MegaStore, uma empresa fictícia de comércio eletrônico com um grande catálogo de produtos.

O objetivo do projeto é utilizar grafos e outras estruturas de dados para representar relações entre produtos e gerar recomendações mais relevantes aos clientes.

O sistema foi desenvolvido em Rust como projeto da disciplina Data Structure Strategy and Implementation.

## Problema

Com o crescimento do catálogo da MegaStore, recomendações baseadas apenas nos produtos mais vendidos ou na categoria dos produtos deixam de representar adequadamente os interesses dos clientes.

Para solucionar esse problema, os produtos são representados como vértices de um grafo e suas relações são representadas por arestas.

Dessa forma, o sistema consegue percorrer as conexões entre os produtos e encontrar itens relacionados.

## Funcionalidades

O projeto possui:

- cadastro de produtos;
- consulta de produtos por ID;
- representação das relações entre produtos através de um grafo;
- recomendação de produtos diretamente relacionados;
- recomendação através do algoritmo BFS;
- prevenção de produtos duplicados durante o percurso;
- testes unitários;
- teste de integração;
- teste básico de desempenho.

## Tecnologias e estruturas utilizadas

### Rust

O sistema foi desenvolvido utilizando a linguagem Rust.

### HashMap

O `HashMap` é utilizado para armazenar os produtos através de seus identificadores e também para representar a lista de adjacência do grafo.

Isso permite acesso rápido aos produtos e às conexões existentes.

### Vec

O `Vec` é utilizado para armazenar as conexões de cada vértice do grafo e também os resultados encontrados durante o percurso.

### VecDeque

O `VecDeque` funciona como fila durante a execução do algoritmo BFS.

### HashSet

O `HashSet` registra os vértices já visitados pelo BFS, evitando visitas repetidas e recomendações duplicadas.

## Modelagem do grafo

Cada produto é representado como um vértice.

As arestas representam relações entre produtos.

Exemplo simplificado:

```text
Notebook
├── Mouse Gamer
└── Teclado Mecânico
    └── Monitor
        └── Webcam

Mouse Gamer
└── Headset
```

O projeto utiliza uma lista de adjacência para representar o grafo.

Exemplo:

```text
1 -> [2, 3]
2 -> [1, 5]
3 -> [1, 4]
4 -> [3, 6]
5 -> [2]
6 -> [4]
```

A lista de adjacência foi escolhida porque é mais adequada para grafos esparsos, evitando o armazenamento de inúmeras posições sem conexão que ocorreria em uma matriz de adjacência.

## Algoritmo BFS

O sistema utiliza o algoritmo Breadth-First Search (BFS), ou Busca em Largura.

O BFS começa em um produto e visita os produtos diretamente conectados a ele. Depois, continua percorrendo as conexões dos produtos encontrados.

Por exemplo, iniciando pelo Notebook, o sistema pode encontrar:

```text
Notebook
↓
Mouse Gamer e Teclado Mecânico
↓
Headset e Monitor
↓
Webcam
```

Assim, o sistema consegue descobrir produtos relacionados além das conexões imediatas.

A complexidade de tempo do BFS é:

```text
O(V + E)
```

onde `V` representa o número de vértices e `E` o número de arestas.

## Arquitetura

O projeto está organizado da seguinte maneira:

```text
megastore/
├── src/
│   ├── main.rs
│   ├── produto.rs
│   ├── grafo.rs
│   └── lib.rs
├── tests/
│   ├── integracao.rs
│   └── desempenho.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### main.rs

Responsável pela execução principal do sistema, cadastro dos produtos, criação das conexões e apresentação das recomendações.

### produto.rs

Contém a estrutura utilizada para representar um produto.

### grafo.rs

Contém o algoritmo BFS utilizado para percorrer o grafo.

### lib.rs

Disponibiliza os módulos do projeto para utilização nos testes de integração.

### tests/

Contém os testes de integração e desempenho.

## Como executar

É necessário possuir Rust e Cargo instalados.

Na pasta do projeto, execute:

```bash
cargo run
```

O programa exibirá os produtos cadastrados, as conexões do grafo e as recomendações.

## Executando os testes

Para executar todos os testes:

```bash
cargo test
```

Para executar o teste de desempenho exibindo os tempos:

```bash
cargo test teste_desempenho_bfs -- --nocapture
```

## Exemplo de execução

```text
====================================
       CONECTASTORE - MEGASTORE
====================================
Sistema de Recomendacao de Produtos
Baseado em Grafos

Recomendações para quem se interessou pelo Notebook:
- Mouse Gamer
- Teclado Mecânico

Recomendações usando BFS:
- Mouse Gamer
- Teclado Mecânico
- Headset
- Monitor
- Webcam
```

## Testes de desempenho

Foram realizados testes do BFS com grafos contendo diferentes quantidades de vértices:

- 100 vértices;
- 1.000 vértices;
- 10.000 vértices.

No teste realizado com 10.000 vértices, o BFS encontrou 9.999 vértices relacionados em aproximadamente 9,52 ms.

Os resultados demonstram que o tempo de execução cresce conforme aumenta o número de vértices e arestas percorridos, de acordo com a complexidade O(V + E) do BFS.

Os valores podem variar de acordo com o computador e entre diferentes execuções.

## Escalabilidade

A utilização de lista de adjacência evita o consumo de memória de uma matriz completa de conexões.

O `HashMap` permite acesso eficiente aos produtos e às listas de conexões, enquanto o BFS permite percorrer apenas os vértices e arestas necessários.

Essas características tornam a solução adequada como demonstração de uma arquitetura que pode ser expandida para catálogos maiores.

Em um sistema real de comércio eletrônico, outras informações poderiam ser adicionadas ao grafo, como clientes, categorias, avaliações, compras e pesos para representar a força das relações.

## Vídeo pitch

Link do vídeo:

[Assistir ao vídeo pitch no YouTube](https://youtu.be/vRMol6Qo1VQ)

## Autor

Projeto acadêmico desenvolvido para a disciplina Data Structure Strategy and Implementation.