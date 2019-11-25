mod cli;

use zflib;
use cli::lztq::parse_cli;


// cmd
//  lztq -i file.fq -o file.fq.lzt -a c
//  lztq -i file.fq.lzt -a d -o file.fq
//  lztq -i file.fq.lzt -a e -o file.fq -l list.csv -d bi
//  lztq -i file.fq.lzt -a e -o file.fq -l "rand(15)" -d fwd

fn main(){

    let cli = parse_cli();

    println!("{:#?}", cli);

}
