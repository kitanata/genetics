extern crate rand;

use rand::{thread_rng, Rng};

fn fitness(target: &Vec<char>, entity: &Vec<char>) -> i32 {
    let mut fit_value = 0;

    for i in 0..50 {
        if(target[i] == entity[i]) {
            fit_value += 1;
        }
    }

    return 50 - fit_value;
}

fn breed(mother: &Vec<char>, father: &Vec<char>) -> Vec<char> {
    let mut child = vec!['o';50];

    for i in 0..50 {
        if mother[i] == father[i] {
            child[i] = 'o';
        } else if mother[i] == 'X' || father[i] == 'X' {
            child[i] = 'X';
        }
    }

    return child;
}

fn mutate(entity: &mut Vec<char>) -> Vec<char> {
    let mut rng = thread_rng();
    for i in 0..5 {
        let rnd_index = rng.gen_range(0, 50);
        let choices = ['X', 'o'];
        entity[rnd_index] = *rng.choose(&choices).unwrap();
    }
    return entity.clone();
}

fn generate_dna() -> Vec<char> {
    let mut new_dna = vec!['o';50];
    let choices = ['X', 'o'];
    let mut rng = thread_rng();
    for x in 0..50 {
        let choice = rng.choose(&choices);

        match choice {
            Some(ref m) => new_dna[x] = **m,
            None => ()
        }
    }

    return new_dna;
}

fn dna_to_string(entity: &Vec<char>) -> String {
    let mut result = String::new();

    for s in entity {
        result.push(*s);
    }

    return result;
}

fn main() {
    let target = generate_dna();
    let adam = generate_dna();
    let eve = generate_dna();

    println!("Target: {}", dna_to_string(&target));
    println!("Adam:   {}", dna_to_string(&adam));
    println!("Eve:    {}", dna_to_string(&eve));

    let mut generation = vec![adam, eve];
    let mut rng = thread_rng();

    let mut generate_num = 0;
    let mut run_generations = true;
    while(run_generations) {
        generate_num += 1;

        for j in 0..5 {
            let mut rnd_index = 0;
            if generation.len() > 2 {
                rnd_index = rng.gen_range(0, generation.len() - 2);
            }

            let father = generation.remove(rnd_index);
            let mother = generation.remove(rnd_index);
            let child_dna = mutate(&mut breed(&mother, &father));

            println!("New Generation! {} Child #: {}", generate_num, j);
            println!("Mother:  {}", dna_to_string(&mother));
            println!("Father:  {}", dna_to_string(&father));
            println!("Child:   {}", dna_to_string(&child_dna));
            println!("Target:  {}", dna_to_string(&target));

            if child_dna == target {
                println!("SUCCESS! Generation #: {} Child #: {}", generate_num, j);
                println!("Child:   {}", dna_to_string(&child_dna));
                println!("Target:  {}", dna_to_string(&target));
                run_generations = false;
                break;
            } else {
                generation.push(child_dna);
                generation.push(father);
                generation.push(mother);
            }
        }

        &generation[..].sort_by(|a, b| fitness(&target, &a).cmp(&fitness(&target, &b)));

        for s in 0..5 {
            generation.pop();
        }
    }
}
