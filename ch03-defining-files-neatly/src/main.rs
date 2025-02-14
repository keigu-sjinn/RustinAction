#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let tmp_length = tmp.len();
        save_to.reserve(tmp_length);
        save_to.append(&mut tmp);
        tmp_length
    }
}
#[allow(unused_variables)]
fn open(f: &mut File) -> bool {
    true
}

#[allow(unused_variables)]
fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer = vec![];
    open(&mut f3);
    let f3_length = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long.", f3.name, f3_length);
    println!("{}", text);
}
