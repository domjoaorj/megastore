use std::collections::HashMap;
use megastore::grafo::bfs;

#[test]
fn teste_recomendacao_integracao() {
    let mut grafo = HashMap::new();

    grafo.insert(1, vec![2, 3]);
    grafo.insert(2, vec![1, 4]);
    grafo.insert(3, vec![1]);
    grafo.insert(4, vec![2]);

    let recomendacoes = bfs(&grafo, 1);

    assert!(recomendacoes.contains(&2));
    assert!(recomendacoes.contains(&3));
    assert!(recomendacoes.contains(&4));
    assert!(!recomendacoes.contains(&1));
}