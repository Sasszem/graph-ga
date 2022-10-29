use std::{time::{Duration, Instant}, fs::File};
use std::io::prelude::*;
use crate::network_raw::{mutation::{CHOICES_MOD, CHOICES_SETTLE}, crossover::*};
use duct::cmd;
use rayon::prelude::*;
use crate::{network_raw::{circuit::Circuit, mutation::*}};
use rand::{seq::SliceRandom, thread_rng};


fn mutate_and_crossover(pool: Vec<Circuit>, size: usize, settle: bool, fitf: fn(&Circuit)->f64) -> (Vec<Circuit>, f64, f64, Duration, Duration, Duration, (Circuit, f64)) {
    let pool = pool;
    let t_fit = Instant::now();
    let mut m : Vec<_> = pool.par_iter().map(|x| (x, fitf(x))).filter(|(_, f)| f.is_normal()).collect();
    
    if m.len() < 20 {
        println!("Too few elements - had {} items to begin with", pool.len());
        pool.par_iter().map(|x| (x, fitf(x))).for_each(
            |x| println!("has fitness: {}", x.1)
        );
        assert!(false);
    }
    
    let t_fit = t_fit.elapsed();
    m.sort_by(|f, s| f.1.partial_cmp(&s.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut out_pool: Vec<Circuit> = Vec::new();

    for k in 0..((size / 10).min(m.len())) {
        out_pool.push(m[k].0.clone());
    }

    
    
    m.truncate(m.len() - 20);
    
    let avg_fitness : f64 = m.iter().map(|(_, f)| f).sum::<f64>() / pool.len() as f64;    
    let t_cross = Instant::now();
    
    let filtered : Vec<_> = m.iter().filter_map(|item| if 1000.0/item.1 > 0.0 {Some((item.0, 1000.0/item.1))} else {None}).collect();
    let min_fitness : f64 = m.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().1;
    let t_mut = Instant::now();
    for _ in 0..((size - out_pool.len()) / 2 ) {
        let mut ckt = filtered.choose_weighted(&mut thread_rng(), |item| item.1).unwrap().0.clone();
        for _ in if settle {0..3} else {0..7} {
            do_mutation_n_tries(&mut ckt, 10, if settle {&CHOICES_SETTLE} else {&CHOICES_MOD});
        }
        out_pool.push(ckt);
    }
    let t_mut = t_mut.elapsed();
    
    if filtered.len() == 0 {
        println!("Not enough items - had {} items to begin with", m.len());
        assert!(false);
    }

    while out_pool.len() < size {
        let first = filtered.choose_weighted(&mut thread_rng(), |item| item.1).unwrap();
        let second = filtered.choose_weighted(&mut thread_rng(), |item| item.1).unwrap();
        let (f2, s2) = crossover_2(first.0, second.0);
        // replicate them 3 times
        for _ in 0..3 {
            out_pool.push(f2.clone());
            out_pool.push(s2.clone());
        }
    }
    out_pool.truncate(size);
    let t_cross = t_cross.elapsed();
    (out_pool, avg_fitness, min_fitness, t_mut, t_fit, t_cross, (m[0].0.clone(), m[0].1))
}

pub fn run_with_ngspice(ckt: &Circuit, commands: &str) -> Vec<(f64, f64)> {
    let mut ckt = ckt.clone();
    mut_simplify(&mut ckt);
    let (spice_desc, node) = ckt.as_spice();
    let stdin = spice_desc + commands + &format!("\n.print ac vm({})\n", node.id);
    let mut res = Vec::new();
    let cmd_res = cmd!("ngspice", "-b").stdin_bytes(stdin).stderr_capture().stdout_capture().run();
    if let Ok(out) = cmd_res {
        // println!("Exit status: {}", out.status);
        let mut capture = 0;
        for line in out.stdout.lines() {
            let line = line.unwrap();
            if line.starts_with("----") {
                capture += 1;
                continue;
            }
            if capture >= 2 {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let (f, r) : (Result<f64, _>, Result<f64, _>) = (parts[1].parse(), parts[2].parse());
                    if let (Ok(f), Ok(r)) = (f, r) {
                        res.push((f, r));
                    }
                }
            }
        }
    }
    return res;
}

pub fn do_ga(base_ckt: &Circuit, n_gen: u32, pool_size: usize, fitf: fn(&Circuit)->f64, printf: fn(&Circuit)) {
    let mut pool : Vec<Circuit> = Vec::new();
    for _ in 0..pool_size {
        let mut ckt_2 = base_ckt.clone();
        for _ in 0..500 {
            do_mutation_n_tries(&mut ckt_2, 10, &CHOICES_MOD);
        }
        mut_simplify(&mut ckt_2);
        pool.push(ckt_2);
    }
    let t = Instant::now();
    let mut avg_t_mut : Duration = Duration::new(0, 0);
    let mut avg_t_fit : Duration = Duration::new(0, 0);
    let mut avg_t_cross : Duration = Duration::new(0, 0);
    let mut pb = kdam::tqdm!(total = n_gen.try_into().unwrap());
    let mut best_ckt_so_far = (Circuit::new(), f64::INFINITY);
    pb.set_postfix(format!("str={}, lst={:?}", "h", [1, 2]));
    kdam::BarExt::refresh(&mut pb);

    let mut file = File::create("fitness_gen.csv").unwrap();
    for i in 0..n_gen {
        if i%50 == 0 {
            println!("At gen {i}");
            for k in pool.iter() {
                println!("Speciment in pool");
                println!("{}", k.to_string());
                println!("-----------------");
            }
        }
        let (out_pool, avg_fitness, min_fitness, t_mut, t_fit, t_cross, best_in_gen) = mutate_and_crossover(pool, pool_size, i > 300, fitf);
        pool = out_pool;
        avg_t_mut += t_mut / n_gen;
        avg_t_fit += t_fit / n_gen;
        avg_t_cross += t_cross / n_gen;
        pb.set_description(format!("GEN {} {} {} {}", i, avg_fitness, min_fitness, best_ckt_so_far.1));
        file.write(format!("{}; {}; {}; {:?}; {:?}; {:?}\n", i, avg_fitness, min_fitness, t_mut.as_secs_f64(), t_fit.as_secs_f64(), t_cross.as_secs_f64()).as_bytes()).unwrap();
        kdam::BarExt::update(&mut pb, 1);
        
        if best_in_gen.1 < best_ckt_so_far.1 {
            let mut ckt_checkpoint = File::create("checkpoint").unwrap();
            best_ckt_so_far = best_in_gen;
            ckt_checkpoint.write(format!("----------------\nFitness: {}\n{}\n", best_ckt_so_far.1, best_ckt_so_far.0.as_spice().0).as_bytes()).unwrap();
            ckt_checkpoint.flush().unwrap();
            if best_ckt_so_far.1 < 1000.0 {
                println!("Found best ckt");
                break;
            }
        }
    }
    eprint!("\n");
    
    // let mut res : Vec<_> = pool.iter().map(|ckt| (ckt, fitf(ckt))).collect();
    // res.sort_by(|f, s| f.1.partial_cmp(&s.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("Timings:");
    println!("   Evolution took {:?}", t.elapsed());
    println!("   Mutation average: {:?}", avg_t_mut);
    println!("   Fitness average: {:?}", avg_t_fit);
    println!("   Crossover average: {:?}", avg_t_cross);
    println!("   GEN = {n_gen}");
    println!("   pool_size = {pool_size}");
    
    let mut ckt = best_ckt_so_far.0;
    mut_simplify(&mut ckt);
    println!("Best ckt so far: {}", ckt.to_string());
    println!("Fitnesse: {}", best_ckt_so_far.1);
    
    printf(&ckt);
    // println!("{}", ckt);
}