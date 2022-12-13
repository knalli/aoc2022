use std::borrow::Borrow;
use std::collections::HashMap;

pub type Fd = i32;

pub struct Filesystem {
    root: Fd,
    data: HashMap<Fd, File>,
    counter: i32,
}

impl Filesystem {
    pub fn new() -> Self {
        let root = File::new_root(0, "/");
        let mut data: HashMap<Fd, File> = HashMap::new();
        data.insert(0, root);
        Filesystem {
            root: 0,
            data: data,
            counter: 1,
        }
    }

    pub fn root(&self) -> Fd {
        self.root
    }

    pub fn parent(&self, fd: Fd) -> Option<Fd> {
        if fd == self.root {
            None
        } else {
            if let Some(f) = self.get(fd) {
                if let Some(p_fd) = f.parent() {
                    Some(p_fd)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn find(&self, context: Fd, name: String) -> Option<Fd> {
        if let Some(dir) = self.get(context) {
            for fd in &dir.children {
                if let Some(f) = self.get(*fd) {
                    if f.name == name {
                        return Some(f.fd);
                    }
                }
            }
        }
        None
    }

    pub fn get(&self, fd: Fd) -> Option<&File> {
        self.data.get(&fd)
    }

    pub fn get_mut(&mut self, fd: Fd) -> Option<&mut File> {
        self.data.get_mut(&fd)
    }

    pub fn _ls(&self, context: Fd) -> Vec<&File> {
        if let Some(dir) = self.get(context) {
            dir.children.iter()
                .map(|f| {
                    self.get(f.clone()).unwrap()
                })
                .collect()
        } else {
            vec!()
        }
    }

    pub fn size_total(&self, fd: Fd) -> Option<i32> {
        if let Some(dir) = self.get(fd) {
            Some(self.size_total_0(dir))
        } else {
            None
        }
    }

    fn size_total_0(&self, file: &File) -> i32 {
        if file.is_dir {
            let mut result = 0;
            for fd in file.children.clone() {
                if let Some(f) = self.get(fd) {
                    result = result + self.size_total_0(f);
                }
            }
            result
        } else {
            file.size
        }
    }

    fn append(&mut self, context: Fd, item: File) {
        let item_fd = item.borrow().fd;
        self.data.insert(item.fd, item);
        self.get_mut(context).unwrap().append(item_fd);
    }

    fn _remove(&mut self, context: Fd, item: Fd) {
        self.get_mut(context).unwrap()._remove(item.clone());
        self.data.remove(&item);
    }

    pub fn mkdir(&mut self, context: Fd, name: &str) -> Fd {
        let fd = self.get_next_fd();
        let dir = File::new_dir(fd, context, name);
        self.append(context, dir);
        fd
    }

    pub fn add_file(&mut self, context: Fd, name: &str, size: i32) -> Fd {
        let fd = self.get_next_fd();
        let file = File::new_file(fd, context, name, size);
        self.append(context, file);
        fd
    }

    fn get_next_fd(&mut self) -> i32 {
        let fd = self.counter;
        self.counter = self.counter + 1;
        fd
    }

    pub fn each(&self, f: impl Fn(Fd) -> ()) {
        let mut stack: Vec<Fd> = vec![self.root];
        while let Some(fd) = stack.pop() {
            let &cwd = &self.get(fd).unwrap();
            f(fd);
            let mut child_fds: Vec<_> = cwd.children.clone().into_iter().collect();
            child_fds.reverse();
            for child_fd in child_fds {
                stack.push(child_fd);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut stack: Vec<(Fd, usize)> = vec![(self.root, 0)];
        let mut result: String = "".to_string();

        while let Some((fd, ident)) = stack.pop() {
            result = result + "\n";
            let &cwd = &self.get(fd).unwrap();
            result = result + (" ".repeat(ident).as_str()) + "- " + cwd.name.as_str();
            if cwd.is_dir {
                result = result + " (dir)";
            } else {
                let s = i32::to_string(&cwd.size());
                result = result + " (file, size=" + s.as_str() + ")";
            }

            let mut child_fds: Vec<_> = cwd.children.clone().into_iter().collect();
            child_fds.reverse();
            for child_fd in child_fds {
                stack.push((child_fd, ident + 1));
            }
        }

        result.to_string()
    }
}

pub struct File {
    fd: Fd,
    parent: Option<Fd>,
    name: String,
    is_dir: bool,
    children: Vec<Fd>,
    size: i32,
}

impl File {
    fn new_root(fd: Fd, name: &str) -> Self {
        File {
            fd,
            parent: None,
            name: name.to_string(),
            is_dir: true,
            children: vec!(),
            size: 0,
        }
    }

    pub fn new_file(fd: Fd, parent: Fd, name: &str, size: i32) -> Self {
        File {
            fd,
            parent: Some(parent),
            name: name.to_string(),
            is_dir: false,
            children: vec!(),
            size: size,
        }
    }

    pub fn new_dir(fd: Fd, parent: Fd, name: &str) -> Self {
        File {
            fd,
            parent: Some(parent),
            name: name.to_string(),
            is_dir: true,
            children: vec!(),
            size: 0,
        }
    }


    pub fn _name(&self) -> &str {
        &self.name
    }

    pub fn children(&self) -> Vec<Fd> {
        self.children.clone()
    }

    fn append(&mut self, fd: Fd) {
        self.children.push(fd);
    }

    fn _remove(&mut self, fd: Fd) {
        for i in 0..self.children.len() {
            if self.children[i] == fd {
                self.children.remove(i);
                break;
            }
        }
    }

    pub fn parent(&self) -> Option<Fd> {
        if let Some(root) = &self.parent {
            Some(root.clone())
        } else {
            None
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

#[test]
fn fs_demo() {
    let mut fs = Filesystem::new();
    let root = fs.root();
    println!("FS DEMO");
    println!("{}", fs.get(root).unwrap()._name());
    let demo = fs.mkdir(root, "demo");
    fs.mkdir(demo, "x");
    fs.add_file(demo, "abc", 123);
    fs.add_file(demo, "abc1", 126);
    println!("ls");
    for d in fs._ls(demo) {
        println!("{}", d.name);
    }
    println!("size = {}", fs.size_total(root).unwrap());
    fs._remove(root, demo);
    println!("size = {}", fs.size_total(root).unwrap());
}
