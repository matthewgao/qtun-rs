extern crate tun;
use std::collections::VecDeque;
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

#[derive(Debug)]
pub struct PacketManger<'a> {
    queue: PacketQueue,
    dev: &'a tun::Device<Queue = Queue>,
}

impl PacketManger {
    fn new() -> Self {
        let mut config = tun::Configuration::default();
        config.address((10, 0, 0, 1))
               .netmask((255, 255, 255, 0))
               .up();
    
        #[cfg(target_os = "macos")]
        config.platform(|config| {
            // config.packet_information(true);
        });


        let mut pm = PacketManger {
            queue: PacketQueue::new(),
            dev: &tun::create(&config).unwrap(),
        };

        // pm.init();
        return pm;
    }

    fn init(&mut self) {
        
    }
}
