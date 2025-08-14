use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::from("helloйd");

    for c in s1.graphemes(true) {
        println!("{c}");
    }
}
