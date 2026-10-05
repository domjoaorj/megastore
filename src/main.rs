use std::collections::HashMap;

mod produto;
use produto::Produto;

mod grafo;
use grafo::bfs;

fn main() {
        println!("====================================");
    println!("       CONECTASTORE - MEGASTORE");
    println!("====================================");
    println!("Sistema de Recomendacao de Produtos");
    println!("Baseado em Grafos\n");
    let mut produtos: HashMap<u32, Produto> = HashMap::new();

    produtos.insert(
        1,
        Produto {
            id: 1,
            nome: String::from("Notebook"),
            categoria: String::from("Eletrônicos"),
            preco: 3500.00,
        },
    );

    produtos.insert(
        2,
        Produto {
            id: 2,
            nome: String::from("Mouse Gamer"),
            categoria: String::from("Eletrônicos"),
            preco: 150.00,
        },
    );

    produtos.insert(
        3,
        Produto {
            id: 3,
            nome: String::from("Teclado Mecânico"),
            categoria: String::from("Eletrônicos"),
            preco: 300.00,
        },
    );
    produtos.insert(
        4,
        Produto {
            id: 4,
            nome: String::from("Monitor"),
            categoria: String::from("Eletrônicos"),
            preco: 1200.00,
        },
    );

    produtos.insert(
        5,
        Produto {
            id: 5,
            nome: String::from("Headset"),
            categoria: String::from("Eletrônicos"),
            preco: 250.00,
        },
    );

    produtos.insert(
        6,
        Produto {
            id: 6,
            nome: String::from("Webcam"),
            categoria: String::from("Eletrônicos"),
            preco: 200.00,
        },
    );
    println!("Produtos cadastrados:");

    for produto in produtos.values() {
        println!(
            "ID: {} | Nome: {} | Categoria: {} | Preço: R$ {:.2}",
            produto.id,
            produto.nome,
            produto.categoria,
            produto.preco
        );
    }    println!("\nConsultando produto de ID 2:");

    match produtos.get(&2) {
        Some(produto) => {
            println!("Produto encontrado: {}", produto.nome);
        }
        None => {
            println!("Produto não encontrado.");
        }
    }
        let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

    grafo.insert(1, vec![2, 3]);
grafo.insert(2, vec![1, 5]);
grafo.insert(3, vec![1, 4]);
grafo.insert(4, vec![3, 6]);
grafo.insert(5, vec![2]);
grafo.insert(6, vec![4]);

    println!("\nConexões do grafo:");

    for (produto_id, relacionados) in &grafo {
        println!("Produto {} está conectado a {:?}", produto_id, relacionados);
    }
        println!("\nRecomendações para quem se interessou pelo Notebook:");

    if let Some(relacionados) = grafo.get(&1) {
        for id_relacionado in relacionados {
            if let Some(produto) = produtos.get(id_relacionado) {
                println!("- {} | R$ {:.2}", produto.nome, produto.preco);
            }
        }
    }
        println!("\nRecomendações usando BFS:");

    let recomendacoes = bfs(&grafo, 1);

    for id in recomendacoes {
        if let Some(produto) = produtos.get(&id) {
            println!("- {}", produto.nome);
        }
    }
}