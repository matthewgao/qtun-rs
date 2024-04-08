extern crate tun;
use std::{collections::VecDeque, rc::Rc};
use tun::platform::Queue;

#[derive(Debug)]
struct PacketQueue {
    queue: VecDeque<Vec<u8>>,
}


impl PacketQueue {
    fn new() -> Self {
        PacketQueue {
            queue : VecDeque::new(),
        }
    }

    // 入队操作：向队尾添加一个元素
    fn enqueue(&mut self, item: Vec<u8>) {
        self.queue.push_back(item);
    }

    // 出队操作：从队首移除一个元素并返回
    fn dequeue(&mut self) -> Option<Vec<u8>> {
        self.queue.pop_front()
    }

    // 获取队列长度
    fn len(&self) -> usize {
        self.queue.len()
    }

    // 检查队列是否为空
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub struct PacketManger {
    queue: PacketQueue,
    dev: Rc<tun::Device<Queue = Queue>>,
}

impl PacketManger {
    pub fn new() -> Self {
        let mut config = tun::Configuration::default();
        config.address((10, 0, 0, 1))
               .netmask((255, 255, 255, 0))
               .up();
    
        #[cfg(target_os = "macos")]
        config.platform(|config| {
            // config.packet_information(true);
        });


        let pm = PacketManger {
            queue: PacketQueue::new(),
            dev: Rc::new(tun::create(&config).unwrap()),
        };

        // pm.init();
        return pm;
    }

    pub fn read_packet(&mut self) {
        let mut buf = [0; 4096];
        loop {
            let amount = Rc::get_mut(&mut self.dev).unwrap().read(&mut buf).unwrap();
            println!("{:?}", &buf[0 .. amount]);
        }
    }
}
