fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {   
        // your code here
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
