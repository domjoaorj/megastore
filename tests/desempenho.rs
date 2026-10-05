use std::collections::HashMap;
use std::time::Instant;
use megastore::grafo::bfs;

#[test]
fn teste_desempenho_bfs() {
    let tamanhos = [100, 1_000, 10_000];

    for tamanho in tamanhos {
        let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

        // Cria um grafo em sequência:
        // 1 -> 2 -> 3 -> 4 -> ...
        for i in 1..tamanho {
            grafo.entry(i).or_default().push(i + 1);
            grafo.entry(i + 1).or_default().push(i);
        }

        let inicio = Instant::now();

        let resultado = bfs(&grafo, 1);

        let tempo = inicio.elapsed();

        println!(
            "Grafo com {} vertices: {} recomendacoes encontradas em {:?}",
            tamanho,
            resultado.len(),
            tempo
        );
    }
}