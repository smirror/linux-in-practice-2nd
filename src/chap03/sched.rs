extern crate chrono;
extern crate nix;

use std::env;
use std::io::{self, BufWriter, Write};
use std::process::exit;

use chrono::{DateTime, Local};
use nix::unistd::{fork, ForkResult};

static NLOOP_FOR_ESTIMATION: i64 = 100_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("使い方: {} <プロセス数>
                   * 論理CPU0上で<プロセス数>の数だけ同時に100ミリ秒程度CPUリソースを消費する負荷処理プロセスを起動した後に、すべてのプロセスの終了を待つ。
                   * \"sched-<プロセス数>.jpg\"というファイルに実行結果を示したグラフを書き出す。
                   * グラフのx軸は負荷処理プロセス開始からの経過時間[ミリ秒]、y軸は進捗[%]", args[0]);
        exit(1);
    }

    let nproc = unwrap_and_validate_arg("nproc", &args[1]);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    writeln!(stdout, "estimating workload which takes just one milliseconds").expect("writeln err(33)");
    let nloops_per_msec: i64 = estimate_loops_per_msec();
    writeln!(stdout, "end estimation, {}", nloops_per_msec).expect("writeln err(35)");
    stdout.flush().expect("flash err(36)");

    let start: DateTime<Local> = Local::now();

    for id in 0..nproc {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _, .. }) => {}
            Ok(ForkResult::Child) => {
                child_fn(&mut stdout, id, nloops_per_msec, start);
            }
            Err(_) => println!("fork failed"),
        }
    }
}

fn unwrap_and_validate_arg(arg_name: &str, arg: &str) -> i8 {
    match arg.parse::<i8>() {
        Ok(value) => {
            if value < 1 {
                eprintln!("<{}>({}) should be => 1", arg_name, value);
                exit(1);
            }
            value
        }
        Err(value) => {
            eprintln!("{} is wrong argument. {} needs int8 not 0", value, arg_name);
            exit(1);
        }
    }
}

fn estimate_loops_per_msec() -> i64 {
    let before: DateTime<Local> = Local::now();
    for _ in 0..NLOOP_FOR_ESTIMATION {}
    let after: DateTime<Local> = Local::now();
    let diff_msec: i64 = after.timestamp_millis() - before.timestamp_millis();
    NLOOP_FOR_ESTIMATION / diff_msec
}

fn child_fn(stdout: &mut dyn Write, id: i8, nloops_per_msec: i64, start: DateTime<Local>) {
    let mut progress = Vec::with_capacity(100);
    let mut now: DateTime<Local>;
    for _ in 0..100 {
        for _ in 0..nloops_per_msec {};
        now = Local::now();
        progress.push(now);
    }
    for (i, now) in progress.iter().enumerate() {
        let diff_msec: i64 = now.timestamp_millis() - start.timestamp_millis();
        writeln!(stdout, "{}\t{}\t{}", id, diff_msec, (i + 1) * 100).expect("err");
    }
    stdout.flush().expect("flush err (92)");
    exit(1);
}
