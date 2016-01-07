extern crate rand;

use rand::{thread_rng, sample, Rng};

fn fitness(target: &Vec<char>, entity: &Vec<char>) -> i32 {
    let mut fit_value = 0;

    for i in 0..50 {
        if(target[i] == entity[i]) {
            fit_value += 1;
        }
    }

    return fit_value;
}

fn breed(mother: &Vec<char>, father: &Vec<char>) -> Vec<char> {
    let mut child = vec!['o';50];

    for i in 0..25 {
        child[i] = mother[i];
    }

    for i in 25..50 {
        child[i] = father[i];
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

    let mut num_entities = 2;
    let mut generate_num = 0;
    let mut run_generations = true;
    while(run_generations) {
        generate_num += 1;

        let gen_len = generation.len();
        let num_pairs = gen_len / 2;

        let mut pairs: Vec<(Vec<char>, Vec<char>)> = Vec::new();

        println!("New Generation # {}! Mating {} pairs!", generate_num, num_pairs);

        // Hunt for mates.
        rng.shuffle(&mut generation);

        for j in 0..num_pairs {
            let father = generation.pop().unwrap();
            let mother = generation.pop().unwrap();

            pairs.push((father, mother));
        }

        println!("Finished generating pairs.");

        for (mother, father) in pairs {
            num_entities += 8;

            for k in 0..8 {
                let child_dna = mutate(&mut breed(&mother, &father));

                println!("Child #: {} = {}", k, dna_to_string(&child_dna));

                if child_dna == target {
                    println!("SUCCESS! Generation #: {} Child #: {}", generate_num, k);
                    println!("Child:   {}", dna_to_string(&child_dna));
                    println!("Target:  {}", dna_to_string(&target));
                    run_generations = false;
                    break;
                }

                generation.push(child_dna);
            }

            if run_generations == false {
                break;
            }
        }

        let pool_size = generation.len();
        println!("Current size of Gene Pool: {}!", pool_size);

        //Calulating mean fitness
        let mut fit_mean = 0;
        for s in &generation {
            fit_mean += fitness(&target, s);
        }
        fit_mean = fit_mean / (pool_size as i32);
        println!("Mean Fit: {}", fit_mean);

        //Find the median
        generation.sort_by(|a, b| fitness(&target, b).cmp(&fitness(&target, a)));

        //Kill off the weak
        let mut survivor_count = generation.len() / 2;

        if survivor_count > 25 {
            survivor_count = 25; //
        }

        let kill_count = generation.len() - survivor_count;

        println!("Killing off: {} entities!", kill_count);
        generation.truncate(survivor_count);

        if generation.len() < 2 {
            panic!("Should not happen.");
        }
    }

    println!("Total Entities Created {}!", num_entities);
}
