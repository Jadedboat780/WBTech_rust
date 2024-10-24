use crossbeam_channel::{unbounded, Receiver, Sender};
use std::{thread, time::Duration};
use task_manager_system::{TaskManage, TaskManager};

fn main() {
    env_logger::init();

    let (sender, receiver) = unbounded();
    let manager = TaskManager::new(sender.clone());
    manager.set_signal();

    let task_manager_thread = thread::spawn(move || {
        let mut task_id_counter = 1;

        loop {
            match receiver.recv().unwrap() {
                TaskManage::Start(command) => {
                    manager.start_task(task_id_counter, command);
                    task_id_counter += 1;
                }
                TaskManage::Stop(task_id) => {
                    manager.stop_task(task_id);
                }
                TaskManage::Status => {
                    manager.status()
                }
                TaskManage::Shutdown => {
                    manager.stop_all_tasks();
                    std::process::exit(0);
                }
            }
        }
    });

    // Пример работы:
    // sender
    //     .send(TaskManage::Start("echo Hello, World!".into()))
    //     .unwrap();
    // thread::sleep(Duration::from_secs(1));
    // sender.send(TaskManage::Status).unwrap();
    // thread::sleep(Duration::from_secs(1));
    // sender.send(TaskManage::Stop(1)).unwrap();

    task_manager_thread.join().unwrap();
}
