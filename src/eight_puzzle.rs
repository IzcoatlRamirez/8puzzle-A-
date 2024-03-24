pub mod eight_puzzle {
    #[derive(Clone, PartialEq)]
    pub struct EightPuzzle {
        pub dim: i32,
        pub board: [[i32; 3]; 3],
        pub pos: (i32, i32),
        pub goal: [[i32; 3]; 3],
        // pub count: i32,
    }

    impl EightPuzzle {
        pub fn new() -> EightPuzzle {
            EightPuzzle {
                dim: 3,
                board: [[0,7,1], [3, 6, 2], [8, 4, 5]],
                pos: (0, 0),
                goal: [[1, 2, 3], [4, 5, 6], [7, 8, 0]],
                // count: 0,
            }
        }

        pub fn print(&self) {
            println!("-------------");
            for (i, fila) in self.board.iter().enumerate() {
                print!("|");
                for (j, &celda) in fila.iter().enumerate() {
                    if (i as i32, j as i32) == self.pos {
                        print!("   "); // Carácter especial para la posición actual
                    } else {
                        print!(" {} ", celda);
                    }
                    print!("|");
                }
                println!("\n-------------");
            }
        }

        pub fn available_moves(&self) -> Vec<(i32, i32)> {
            let mut moves = Vec::new();
            if self.pos.0 > 0 {
                moves.push((self.pos.0 - 1, self.pos.1));
            }
            if self.pos.0 < self.dim - 1 {
                moves.push((self.pos.0 + 1, self.pos.1));
            }
            if self.pos.1 > 0 {
                moves.push((self.pos.0, self.pos.1 - 1));
            }
            if self.pos.1 < self.dim - 1 {
                moves.push((self.pos.0, self.pos.1 + 1));
            }
            return moves;
        }

        pub fn movepos(&mut self, position: (i32, i32)) {
            let temp = self.pos;
            self.pos = position;
            self.board[temp.0 as usize][temp.1 as usize] =
                self.board[position.0 as usize][position.1 as usize];
            self.board[position.0 as usize][position.1 as usize] = 0; // O el valor que desees
        }

        pub fn is_goal(&self) -> bool {
            return self.board == self.goal;
        }

        pub fn heuristic_cost(&self) -> f64 {
            let mut total_cost = 0.0;
            let mut misplaced_tiles:i32 = 0;
    
            for i in 0..self.dim {
                for j in 0..self.dim {
                    if self.board[i as usize][j as usize] != 0 && self.board[i as usize][j as usize] != self.goal[i as usize][j as usize] {
                        let current_value = self.board[i as usize][j as usize];                 
                        let (goal_i, goal_j) = self.find_goal_position(current_value);
                        let distance = (i.pow(2) as f64 + j.pow(2) as f64).sqrt() - (goal_i.pow(2) as f64 + goal_j.pow(2) as f64).sqrt();
                        total_cost += distance;
                        misplaced_tiles += 1;
                    }
                }
                println!();
            }
    
            return total_cost + misplaced_tiles as f64;
        }

        fn find_goal_position(&self, value: i32) -> (i32, i32) {
            match value {
                1 => (0, 0),
                2 => (0, 1),
                3 => (0, 2),
                4 => (1, 0),
                5 => (1, 1),
                6 => (1, 2),
                7 => (2, 0),
                8 => (2, 1),
                _ => panic!("Invalid value"),
            }
        }

        pub fn solve(&mut self) -> Vec<EightPuzzle> {
            let mut open_list: Vec<(EightPuzzle, f64)> = Vec::new(); // Lista de estados abiertos
            let mut closed_list: Vec<EightPuzzle> = Vec::new(); // Lista de estados cerrados
            let mut solution: Vec<EightPuzzle> = Vec::new(); // Lista de la solución
            
            open_list.push((self.clone(), self.heuristic_cost())); // Agregar el estado inicial con su costo heurístico
            
            while !open_list.is_empty() {
                // Ordenar la lista de estados abiertos por costo heurístico
                open_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                
                // Obtener el estado con el menor costo heurístico
                let (current_state, _) = open_list.remove(0);
                
                // Verificar si el estado actual es el objetivo
                if current_state.is_goal() {
                    solution.push(current_state.clone());
                    return solution;
                }
                
                // Agregar el estado actual a la lista de estados cerrados
                closed_list.push(current_state.clone());
                
                // Generar los movimientos posibles y calcular el costo heurístico para cada uno
                for move_position in current_state.available_moves() {
                    let mut new_state = current_state.clone();
                    new_state.movepos(move_position);
                    
                    // Verificar si el nuevo estado ya está en la lista de estados cerrados
                    if closed_list.contains(&new_state) {
                        continue;
                    }
                    
                    let cost = new_state.heuristic_cost();
                    open_list.push((new_state, cost));
                }
                
                solution.push(current_state);
                // self.count += 1;
            }
            
            return Vec::new(); // No se encontró solución
        }
    }
}
