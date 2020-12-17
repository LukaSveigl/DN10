#include <mutex>
#include <chrono>
#include <thread>
#include <sstream>
#include <iostream>

std::mutex lockThread;

void thread_do_stuff(int id){
    std::lock_guard<std::mutex> lock(lockThread);
    for(unsigned int i = 0; i < 5; i++) {
        std::this_thread::sleep_for(std::chrono::seconds(3));
        std::cout << "Thread " << id << "prints this text!" << std::endl;
    }
}

int main(){
    std::thread test_thread1 (thread_do_stuff, 1);
    test_thread1.join();
    std::thread test_thread2 (thread_do_stuff, 2);
    test_thread2.join();

    return 0;
}