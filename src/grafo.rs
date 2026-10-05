use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(grafo: &HashMap<u32, Vec<u32>>, inicio: u32) -> Vec<u32> {
    let mut visitados = HashSet::new();
    let mut fila = VecDeque::new();
    let mut resultado = Vec::new();

    visitados.insert(inicio);
    fila.push_back(inicio);

    while let Some(atual) = fila.pop_front() {
        if let Some(vizinhos) = grafo.get(&atual) {
            for vizinho in vizinhos {
                if !visitados.contains(vizinho) {
                    visitados.insert(*vizinho);
                    fila.push_back(*vizinho);
                    resultado.push(*vizinho);
                }
            }
        }
    }

    resultado
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_bfs() {
        let mut grafo = HashMap::new();

        grafo.insert(1, vec![2, 3]);
        grafo.insert(2, vec![1, 4]);
        grafo.insert(3, vec![1]);
        grafo.insert(4, vec![2]);

        let resultado = bfs(&grafo, 1);

        assert_eq!(resultado, vec![2, 3, 4]);
    }
}