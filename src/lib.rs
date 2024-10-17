use std::thread;

pub struct ThreadPool{
    threads : Vec<thread::JoinHandle<()>>
}


impl ThreadPool {
    pub fn new (number : usize) -> ThreadPool{

        // new will panic if the number of thread allowed is zero or negative because that has no sense
        assert!(number>0);
        // we will create a vector that should hold the joinhandler that issues from spawn method when a thread is created
        let threads = Vec::with_capacity(number); // we utilized with_capacity for efficiency sake!!!
        for _ in 0..number{
            // logic to create threads and fill in threads vector
        }
        
        ThreadPool { threads}
    }
    pub fn execute<F>(&self,f : F)
    where 
        F:FnOnce() + Send + 'static
    {
        
    }
}