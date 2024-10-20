Main goal : 
Mutlithreaded web server!!
Goal:

Create a thread pool with a limited number of threads to efficiently manage resources and prevent potential DDoS attacks.
Motivation:

The standard library does not provide a native way to create an "empty" thread that simply waits for work. To overcome this, we will use channels to manage the communication between threads and ensure they can wait for tasks without consuming unnecessary resources.
Task Description:

    Design a data structure that facilitates the creation of a thread pool, enabling threads to wait for tasks via channels.
    This structure will allow us to control the number of active threads and distribute tasks in a safe, efficient manner.