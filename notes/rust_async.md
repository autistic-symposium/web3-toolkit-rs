## 🦀 async for rust

<br>

#### on threads vs. async

<br>

* the problems with threads: 1) use lots of memory, 2) launches/context switches are cost when thousands of threads are running
* conversely, `async` and `await` provides concurrency, optimized hardware utilization, betters speed/performance, and lower I/O workloads.


<br>

---

#### on futures

<br>

* the center of async-wait is a feature called future (anything that can be computed asynchronously).
* what does the keyword async in front of fn really mean?

<img width="300" alt="Screen Shot 2023-03-11 at 3 00 36 PM" src="https://user-images.githubusercontent.com/1130416/224515182-6174279b-65dd-4654-adfe-6cbffa8c845a.png">

<br>

----

#### on blocking

<br>

* async/await in rust is implemented using "cooperative scheduling"
* a naive way to write applications is to spawn a new thread for every tasks, which works when the number of tasks is small, but doesn't scale for a large number of threads
* the swap between running tasks on each thread happens with `await`: "blocking the thread" means "preventing the runtime from swapping the current tasks".
* using `[tokio::spawn]` and multi-threaded runtime allow running multiple blocking tasks until you run out of threads.
* the default tokio runtime spawns one thread per cpu core.
