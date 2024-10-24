use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
use std::thread;

/// Тип сообщения для управления задачами
#[derive(Debug)]
pub enum TaskManage {
    Start(String),
    Stop(u32),
    Status,
    Shutdown,
}

/// Задача
#[derive(Debug)]
pub struct Task {
    pub command: String,
    pub handle: Option<thread::JoinHandle<()>>,
    pub running: Arc<AtomicBool>,
}

/// Менеджер задач
#[derive(Debug)]
pub struct TaskManager {
    pub tasks: Arc<Mutex<HashMap<u32, Task>>>,
    pub sender: Sender<TaskManage>,
}

impl TaskManager {
    /// Конструктор
    pub fn new(sender: Sender<TaskManage>) -> Self {
        TaskManager {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            sender,
        }
    }

    /// Запуск новой задачи
    pub fn start_task(&self, task_id: u32, command: String) {
        let is_running = Arc::new(AtomicBool::new(true));

        let handle = thread::spawn({
            let command = command.clone();
            let is_running = is_running.clone();
            log::info!("Task {} is running command: {}", task_id, command);

            move || {
                Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .output()
                    .map(|output| log::info!("Task {} finished with output: {:?}", task_id, output))
                    .unwrap_or_else(|e| log::error!("Task {} failed to run: {}", task_id, e));
            }
        });

        let task = Task {
            command,
            handle: Some(handle),
            running: is_running,
        };

        self.tasks.lock().unwrap().insert(task_id, task);
        log::info!("Task {} started", task_id);
    }

    /// Остановка работы задачи
    pub fn stop_task(&self, task_id: u32) {
        self.tasks
            .lock()
            .unwrap()
            .get_mut(&task_id)
            .map(|task| {
                task.running.store(false, Ordering::Release);
                task.handle.take().map(|handle| handle.join().unwrap());
                log::info!("Task {} has been stopped", task_id);
            })
            .unwrap_or_else(|| {
                log::warn!("Task {} not found", task_id);
            });
    }

    pub fn status(&self) {
        let tasks = self.tasks.lock().unwrap();
        log::info!("Currently running tasks: {}", tasks.len());
    }

    /// Остановка работы всех задач
    pub fn stop_all_tasks(&self) {
        log::info!("All tasks has been stopped");
        self.tasks
            .lock()
            .unwrap()
            .keys()
            .for_each(|t| self.stop_task(*t));
    }

    /// Установка сигналов для управления задачами
    pub fn set_signal(&self) {
        ctrlc::set_handler({
            let sender = self.sender.clone();
            move || {
                log::info!("Received Ctrl+C! Shutting down...");
                sender.send(TaskManage::Shutdown).unwrap();
            }
        })
        .unwrap();
    }
}
