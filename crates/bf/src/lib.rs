use std::{collections::HashMap, str::Chars};

#[derive(Debug)]
pub struct State<'a> {
    pub memory: Vec<u8>,
    ptr: usize,
    pub output: String,
    input: Chars<'a>,
}

fn create_bracket_map(programs: &[char]) -> Result<HashMap<usize, usize>, String> {
    let mut bracket_map: HashMap<usize, usize> = HashMap::new();
    let mut start_stack: Vec<usize> = vec![];

    for (i, &c) in programs.iter().enumerate() {
        if c == '[' {
            start_stack.push(i);
        } else if c == ']' {
            let start_index = start_stack
                .pop()
                .ok_or("`]`に対応する`[`が見つかりません。")?;
            bracket_map.insert(start_index, i);
            bracket_map.insert(i, start_index);
        }
    }

    if !start_stack.is_empty() {
        return Err("`[`に対応する`]`が見つかりません。".into());
    };

    Ok(bracket_map)
}

pub fn run<'a>(programs: &'a str, input: &'a str) -> Result<State<'a>, String> {
    let mut state = State {
        memory: vec![0; 1024],
        ptr: 0,
        output: String::new(),
        input: input.chars(),
    };

    let programs: Vec<char> = programs.chars().collect();
    let bracket_map = create_bracket_map(&programs)?;
    // program counter
    let mut pc: usize = 0;

    while pc < programs.len() {
        match programs[pc] {
            '+' => state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1),
            '-' => state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1),
            '>' => state.ptr = state.ptr.wrapping_add(1),
            '<' => {
                if state.ptr != 0 {
                    state.ptr = state.ptr.wrapping_sub(1)
                }
            }
            '.' => state.output.push(state.memory[state.ptr].into()),
            '[' => {
                if state.memory[state.ptr] == 0 {
                    pc = *bracket_map
                        .get(&pc)
                        .ok_or("`[`に対応する`]`が見つかりません。")?;
                    continue;
                }
            }
            ']' => {
                if state.memory[state.ptr] != 0 {
                    pc = *bracket_map
                        .get(&pc)
                        .ok_or("`]`に対応する`[`が見つかりません。")?;
                    continue;
                }
            }
            ',' => {
                state.memory[state.ptr] =
                    state.input.next().ok_or("入力が与えられませんでした。")? as u8;
            }
            _ => {}
        };
        pc += 1;
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let program = "";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
    }

    #[test]
    fn test_increment() {
        let program = "+";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 1);
    }

    #[test]
    fn test_decrement() {
        let program = "-";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 255);
    }

    #[test]
    fn test_multiple_increment() {
        let program = "+++++";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 5);
    }

    #[test]
    fn test_multiple_decrement() {
        let program = "-----";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 251);
    }

    #[test]
    fn test_multiple_increment_and_decrement() {
        let program = "+++-+-";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 2);
    }

    #[test]
    fn test_ptr_move_right() {
        let program = ">";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
        assert_eq!(state.memory[1], 0);
    }

    #[test]
    fn test_ptr_move_right_and_increment() {
        let program = ">+";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
        assert_eq!(state.memory[1], 1);
    }

    #[test]
    fn test_ptr_move_left() {
        let program = "<";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
    }

    #[test]
    fn test_ptr_move_left_and_increment() {
        let program = "<+";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 1);
    }

    #[test]
    fn test_move_right_and_left_and_increment() {
        let program = "><++";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 2);
    }

    // test `.` works correctly
    #[test]
    fn test_output() {
        let program = "+.";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 1);
        assert_eq!(state.output, "\u{1}")
    }

    #[test]
    fn test_output_multiple() {
        let program = "+++.+.";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 4);
        assert_eq!(state.output, "\u{3}\u{4}")
    }

    #[test]
    fn test_output_after_pointer_move() {
        let program = "+>++.<.";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 1);
        assert_eq!(state.memory[1], 2);
        assert_eq!(state.output, "\u{2}\u{1}")
    }

    #[test]
    fn test_loop_noop() {
        let program = "[+]";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
    }

    #[test]
    fn test_loop_single_iteration() {
        let program = "+[->+<]";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
        assert_eq!(state.memory[1], 1);
    }

    #[test]
    fn test_loop_multiple_iterations() {
        let program = "+++[->+<]";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
        assert_eq!(state.memory[1], 3);
    }

    #[test]
    fn test_loop_nested() {
        let program = "++[>++[>+++<-]<-]";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 0);
        assert_eq!(state.memory[1], 0);
        assert_eq!(state.memory[2], 12);
    }

    #[test]
    fn test_input() {
        let program = ",";
        let input = "\u{1}";
        let state = run(program, input).unwrap();
        assert_eq!(state.memory[0], 1);
    }

    #[test]
    fn test_input_multiple() {
        let program = ",>,>,";
        let input = "\u{1}\u{2}\u{3}";
        let state = run(program, input).unwrap();
        assert_eq!(state.memory[0], 1);
        assert_eq!(state.memory[1], 2);
        assert_eq!(state.memory[2], 3);
    }

    #[test]
    fn test_input_invalid() {
        let program = ",>,>,";
        let input = "\u{1}\u{2}";
        let result = run(program, input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "入力が与えられませんでした。");
    }

    #[test]
    fn test_comments() {
        let program = "+a+b>c+.>[,]";
        let state = run(program, "").unwrap();
        assert_eq!(state.memory[0], 2);
        assert_eq!(state.memory[1], 1);
        assert_eq!(state.output, "\u{1}");
    }

    #[test]
    fn test_create_bracket_map_simple() {
        let m = create_bracket_map(&"[]".chars().collect::<Vec<_>>()).unwrap();
        assert_eq!(m.get(&0), Some(&1));
        assert_eq!(m.get(&1), Some(&0));
    }

    #[test]
    fn test_create_bracket_map_nested() {
        let m = create_bracket_map(&"[[]]".chars().collect::<Vec<_>>()).unwrap();
        assert_eq!(m.get(&0), Some(&3));
        assert_eq!(m.get(&1), Some(&2));
        assert_eq!(m.get(&2), Some(&1));
        assert_eq!(m.get(&3), Some(&0));
    }

    #[test]
    fn test_create_bracket_map_multiple() {
        let m = create_bracket_map(&"[][]".chars().collect::<Vec<_>>()).unwrap();
        assert_eq!(m.get(&0), Some(&1));
        assert_eq!(m.get(&1), Some(&0));
        assert_eq!(m.get(&2), Some(&3));
        assert_eq!(m.get(&3), Some(&2));
    }

    #[test]
    fn test_create_bracket_map_unmatched_open() {
        let err = create_bracket_map(&"[[]".chars().collect::<Vec<_>>()).unwrap_err();
        assert_eq!(err, "`[`に対応する`]`が見つかりません。");
    }

    #[test]
    fn test_create_bracket_map_unmatched_close() {
        let err = create_bracket_map(&"[]]".chars().collect::<Vec<_>>()).unwrap_err();
        assert_eq!(err, "`]`に対応する`[`が見つかりません。");
    }
}
